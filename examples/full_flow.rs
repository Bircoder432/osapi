use osapi::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    #[cfg(feature = "logging")]
    osapi::logging::init();
    let client = Client::new("https://api.thisishyum.ru/schedule_api/tyumen/").with_college(1);

    let campuses = client
        .campuses()?
        .name("Луначарского 2 курс")
        .send()
        .await?;

    println!("Найдено кампусов: {}", campuses.len());
    for campus in &campuses {
        println!("  • {} (ID: {})", campus.name, campus.id);
    }

    if let Some(campus) = campuses.first() {
        let groups = client.groups(campus.id).name("ИС-24-2").send().await?;

        println!("\nНайдено групп: {}", groups.len());
        for group in &groups {
            println!("  • {} (ID: {})", group.name, group.id);
        }

        if let Some(group) = groups.first() {
            let today = client.today(group.id).send().await?;

            println!("\nРасписание на сегодня для {}:", group.name);
            for schedule in today {
                println!("  📅 {}: {} занятий", schedule.date, schedule.lessons.len());
                for lesson in schedule.lessons {
                    println!(
                        "    • {} — {} | {}",
                        lesson.start_time, lesson.end_time, lesson.title
                    );
                }
            }
        }
    }

    Ok(())
}
