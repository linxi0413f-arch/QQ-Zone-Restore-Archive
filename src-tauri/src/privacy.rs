use std::{fs, path::PathBuf};

use rusqlite::{params, Connection};
use serde::Serialize;
use tauri::Manager;

use crate::qlogin::QLoginState;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PrivacyStatus {
    local_only: bool,
    telemetry_enabled: bool,
    cloud_storage_enabled: bool,
    credentials_persisted: bool,
    developer_server_used: bool,
    current_uin: Option<String>,
    app_data_dir: String,
    cache_dir: String,
    database_path: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteCurrentAccountResult {
    uin: String,
    deleted_rows: u64,
    media_cache_cleared: bool,
}

fn database_path(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("无法获取应用数据目录：{error}"))?;
    Ok(dir.join("qzone-archive.sqlite3"))
}

fn remove_dir_if_exists(path: &PathBuf, label: &str) -> Result<bool, String> {
    if !path.exists() {
        return Ok(false);
    }
    fs::remove_dir_all(path).map_err(|error| format!("删除{label}失败：{error}"))?;
    Ok(true)
}

fn table_exists(connection: &Connection, table: &str) -> Result<bool, String> {
    connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)",
            params![table],
            |row| row.get::<_, bool>(0),
        )
        .map_err(|error| format!("检查本地数据表失败：{error}"))
}

#[tauri::command]
pub async fn get_privacy_status(
    app: tauri::AppHandle,
    login: tauri::State<'_, QLoginState>,
) -> Result<PrivacyStatus, String> {
    let app_data_dir = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("无法获取应用数据目录：{error}"))?;
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|error| format!("无法获取应用缓存目录：{error}"))?;
    let current_uin = login.qzone_auth().await.ok().map(|auth| auth.uin);

    Ok(PrivacyStatus {
        local_only: true,
        telemetry_enabled: false,
        cloud_storage_enabled: false,
        credentials_persisted: false,
        developer_server_used: false,
        current_uin,
        app_data_dir: app_data_dir.display().to_string(),
        cache_dir: cache_dir.display().to_string(),
        database_path: database_path(&app)?.display().to_string(),
    })
}

#[tauri::command]
pub async fn delete_current_account_data(
    app: tauri::AppHandle,
    login: tauri::State<'_, QLoginState>,
) -> Result<DeleteCurrentAccountResult, String> {
    let owner_uin = login.qzone_auth().await?.uin;
    let database = database_path(&app)?;
    let mut deleted_rows = 0_u64;

    if database.exists() {
        let mut connection = Connection::open(&database)
            .map_err(|error| format!("无法打开本地归档数据库：{error}"))?;
        connection
            .execute_batch("PRAGMA secure_delete=ON; PRAGMA foreign_keys=ON;")
            .map_err(|error| format!("启用安全删除模式失败：{error}"))?;

        let existing_tables = [
            ("archive_feeds", "互动记录"),
            ("archive_dynamics", "动态记录"),
            ("archive_checkpoints", "续传记录"),
            ("archive_rate_limits", "频率记录"),
            ("archive_skips", "异常跳过记录"),
        ]
        .into_iter()
        .filter_map(|(table, label)| match table_exists(&connection, table) {
            Ok(true) => Some(Ok((table, label))),
            Ok(false) => None,
            Err(error) => Some(Err(error)),
        })
        .collect::<Result<Vec<_>, String>>()?;

        let transaction = connection
            .transaction()
            .map_err(|error| format!("开始本地数据删除事务失败：{error}"))?;

        // Interaction rows are removed before dynamics so the order also works
        // if a future schema adds foreign keys between these tables.
        for (table, label) in existing_tables {
            let sql = format!("DELETE FROM {table} WHERE owner_uin=?1");
            let affected = transaction
                .execute(&sql, params![owner_uin])
                .map_err(|error| format!("删除{label}失败：{error}"))?;
            deleted_rows = deleted_rows.saturating_add(affected as u64);
        }

        transaction
            .commit()
            .map_err(|error| format!("提交本地数据删除事务失败：{error}"))?;

        // Secure-delete overwrites deleted cells, then checkpoint/VACUUM rewrites
        // the database so deleted QQ content is not left in free pages or WAL.
        connection
            .execute_batch("PRAGMA wal_checkpoint(TRUNCATE); VACUUM; PRAGMA wal_checkpoint(TRUNCATE);")
            .map_err(|error| format!("压缩并清理本地数据库残留失败：{error}"))?;
    }

    // Older upstream builds store downloaded media in shared cache folders.
    // Purge those folders when deleting one account so no media remnants from
    // the deleted account can remain on disk. Other accounts keep their DB
    // records and can re-fetch media when needed.
    let images = app
        .path()
        .app_data_dir()
        .map_err(|error| format!("无法获取图片归档目录：{error}"))?
        .join("images");
    let videos = app
        .path()
        .app_cache_dir()
        .map_err(|error| format!("无法获取视频缓存目录：{error}"))?
        .join("videos");
    let images_removed = remove_dir_if_exists(&images, "图片缓存")?;
    let videos_removed = remove_dir_if_exists(&videos, "视频缓存")?;

    // Credentials are memory-only. Clearing the session guarantees that
    // deleting account data also removes the active QQ authentication state.
    login.clear_session().await;

    Ok(DeleteCurrentAccountResult {
        uin: owner_uin,
        deleted_rows,
        media_cache_cleared: images_removed || videos_removed,
    })
}
