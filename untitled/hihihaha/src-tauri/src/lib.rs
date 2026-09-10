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

            sql: include_str!("../migrations/0001_inital.sql"),

            kind: MigrationKind::Up,
        },
        Migration{
            version:2,

            description: "create_reactions_table",

            sql: include_str!("../migrations/0002_reactions.sql"),

            kind: MigrationKind::Up,
        },
    ];

    tauri::Builder::default()
        .plugin(
            tauri_plugin_sql::Builder::default()

                .add_migrations("sqlite:messanger.db", migrations)

                .build()
        )
    // Создаем plugin opener
        .plugin(tauri_plugin_opener::init())

        .run(tauri::generate_context!())
        .expect("Ошибка при сборке приложения");
}