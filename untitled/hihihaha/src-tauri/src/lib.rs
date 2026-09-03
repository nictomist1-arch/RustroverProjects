// Импорт типов, еобходимых для migrations
use tauri_plugin_sql::{Migration, MigrationKind};

// Анотация необходимая Tauri для мобильных платформ
// На Win он не мешает
#[cfg_attr(mobile, tauri::mobile_entry_point)]

// Главная функция для запуска приложения
pub fn run(){
    // Создание списка миграций
    let migrations = vec![
        // Описание первой миграции
        Migration{
            version:1,

            description: "create_message_table",

            sql: include_str!("../migrations/0001_init.sql"),

            kind: MigrationKind::Up,
        }
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::default()

                .add_migrations("sqlite:messenger.db", migrations)

                .build()
        )
    // Создаем plugin opener
        .plugin(tauri_plugin_opener::init())

        .run(tauri::generate_context!())
        .expect("Ошибка при сборке приложения");
}