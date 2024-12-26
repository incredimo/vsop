use astro::*;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::Asia::Kolkata;
use prettytable::{format::{self, TableFormat}, Cell, Row, Table};
use colored::*;

fn main() -> Result<()> {
    // Create birth data for Aghil
    let aghils_birth_data: BirthData = BirthData {
        datetime: Kolkata
            .with_ymd_and_hms(1991, 6, 18, 7, 10, 0)
            .unwrap()
            .with_timezone(&Utc),
        longitude: 76.97,
        latitude: 10.80,
    };

    let birth_data = aghils_birth_data.clone();
    let jd = birth_data.to_jd()?;
    let ayanamsa = calculate_ayanamsa(jd);
    let asc_sid_deg = compute_ascendant_sidereal(&birth_data);
    let planets = compute_all_planets(jd)?;
 


    println!("\n{}", "=== VEDIC BIRTH CHART ANALYSIS ===".bold());
    println!("{}", "--------------------------------".bold());
    
    // Basic Information Table
    let mut basic_info = Table::new();
    basic_info.set_format(*format::consts::FORMAT_BOX_CHARS);
    basic_info.add_row(Row::new(vec![
        Cell::new("Name"),
        Cell::new("AGHIL MOHAN"),
    ]));
    basic_info.add_row(Row::new(vec![
        Cell::new("Date"),
        Cell::new("June 18th, 1991"),
    ]));
    basic_info.add_row(Row::new(vec![
        Cell::new("Time"),
        Cell::new("07:10 AM IST"),
    ]));
    basic_info.add_row(Row::new(vec![
        Cell::new("Place"),
        Cell::new("Calicut, Kerala, India"),
    ]));
    basic_info.add_row(Row::new(vec![
        Cell::new("Coordinates"),
        Cell::new(&format!("{}°E, {}°N", birth_data.longitude, birth_data.latitude)),
    ]));
    basic_info.printstd();

    // Technical Details Table
    println!("\n{}", "Technical Details".bold());
    let mut tech_details = Table::new();
    tech_details.set_format(*format::consts::FORMAT_BOX_CHARS);
    tech_details.add_row(Row::new(vec![
        Cell::new("Julian Day"),
        Cell::new(&format!("{:.6}", jd)),
    ]));
    tech_details.add_row(Row::new(vec![
        Cell::new("Ayanamsa"),
        Cell::new(&format!("{:.6}°", ayanamsa * RAD_TO_DEG)),
    ]));
    tech_details.printstd();

    // Ascendant Details
    println!("\n{}", "Ascendant Details".bold());
    let (asc_rasi, asc_deg, asc_min, asc_sec) = rasi_details(asc_sid_deg);
    let mut asc_table = Table::new();
    asc_table.set_format(*format::consts::FORMAT_BOX_CHARS);
    asc_table.add_row(Row::new(vec![
        Cell::new("Sign"),
        Cell::new(&asc_rasi),
    ]));
    asc_table.add_row(Row::new(vec![
        Cell::new("Position"),
        Cell::new(&format!("{}°{}'{:.1}\"", asc_deg, asc_min, asc_sec)),
    ]));
    asc_table.printstd();

    // Planetary Positions with House Placements
    println!("\n{}", "Planetary Positions".bold());
    let mut planet_table = Table::new();
    planet_table.set_titles(Row::new(vec![
        Cell::new("Planet").style_spec("b"),
        Cell::new("Sign").style_spec("b"),
        Cell::new("Position").style_spec("b"),
        Cell::new("House").style_spec("b"),
        Cell::new("Dignity").style_spec("b"),
    ]));

    for planet in &planets {
        if planet.name == "Sun" || planet.name == "Moon" || planet.name == "Rahu" || planet.name == "Ketu" {
            continue;
        }
        let (rasi, deg, min, sec) = rasi_details(planet.sidereal_long_deg);
        let house = ((planet.sidereal_long_deg - asc_sid_deg) / 30.0).floor() as i32 % 12 + 1;
        let dignity = calculate_dignity(planet)?;
        let dignity_status = if dignity.exalted {
            "Exalted"
        } else if dignity.own_sign {
            "Own Sign"
        } else if dignity.debilitated {
            "Debilitated"
        } else if dignity.friendly_sign {
            "Friendly"
        } else if dignity.enemy_sign {
            "Enemy Sign"
        } else {
            "Neutral"
        };

        planet_table.add_row(Row::new(vec![
            Cell::new(&planet.name),
            Cell::new(&rasi),
            Cell::new(&format!("{}°{}'{:.1}\"", deg, min, sec)),
            Cell::new(&format!("H{}", house)),
            Cell::new(dignity_status),
        ]));
    }
    planet_table.printstd();

    // Panchanga
    println!("\n{}", "Panchanga (Five Limbs)".bold());
    let panchanga = compute_panchanga(jd);
    let mut panchanga_table = Table::new();
    panchanga_table.set_format(*format::consts::FORMAT_BOX_CHARS);
    panchanga_table.add_row(Row::new(vec![
        Cell::new("Tithi"),
        Cell::new(&format!("{} {}", panchanga.tithi_number, panchanga.paksha)),
    ]));
    panchanga_table.add_row(Row::new(vec![
        Cell::new("Vara"),
        Cell::new(&panchanga.weekday),
    ]));
    panchanga_table.add_row(Row::new(vec![
        Cell::new("Nakshatra"),
        Cell::new(&format!("{} ({})", panchanga.nakshatra_name, panchanga.nakshatra_index)),
    ]));
    panchanga_table.add_row(Row::new(vec![
        Cell::new("Yoga"),
        Cell::new(&format!("{} ({})", panchanga.yoga_name, panchanga.yoga_index)),
    ]));
    panchanga_table.add_row(Row::new(vec![
        Cell::new("Karana"),
        Cell::new(&format!("{} ({})", panchanga.karana_name, panchanga.karana_index)),
    ]));
    panchanga_table.printstd();

    // House Details
    println!("\n{}", "House Details".bold());
    let houses = compute_whole_sign_houses(asc_sid_deg);
    let mut house_table = Table::new();
    house_table.set_titles(Row::new(vec![
        Cell::new("House").style_spec("b"),
        Cell::new("Sign").style_spec("b"),
        Cell::new("Position").style_spec("b"),
        Cell::new("Planets").style_spec("b"),
    ]));

    for (i, &cusp) in houses.iter().enumerate() {
        let (rasi, deg, min, sec) = rasi_details(cusp);
        let house_num = i + 1;
        
        // Get planets in this house
        let house_planets: Vec<String> = planets
            .iter()
            .filter(|p| {
                let planet_house = ((p.sidereal_long_deg - asc_sid_deg) / 30.0).floor() as usize % 12 + 1;
                planet_house == house_num
            })
            .map(|p| p.name.clone())
            .collect();

        house_table.add_row(Row::new(vec![
            Cell::new(&format!("H{}", house_num)),
            Cell::new(&rasi),
            Cell::new(&format!("{}°{}'{:.1}\"", deg, min, sec)),
            Cell::new(&house_planets.join(", ")),
        ]));
    }
    house_table.printstd();

    // Planetary Strengths
    println!("\n{}", "Planetary Strengths (Shadbala)".bold());
    let mut strength_table = Table::new();
    strength_table.set_titles(Row::new(vec![
        Cell::new("Planet").style_spec("b"),
        Cell::new("Sthana").style_spec("b"),
        Cell::new("Dig").style_spec("b"),
        Cell::new("Kala").style_spec("b"),
        Cell::new("Drik").style_spec("b"),
        Cell::new("Natural").style_spec("b"),
        Cell::new("Total").style_spec("b"),
    ]));

    for planet in &planets {
        if let Ok(strength) = calculate_shadbala(planet, jd, asc_sid_deg) {
            strength_table.add_row(Row::new(vec![
                Cell::new(&planet.name),
                Cell::new(&format!("{:.2}", strength.sthan_bala)),
                Cell::new(&format!("{:.2}", strength.dig_bala)),
                Cell::new(&format!("{:.2}", strength.kala_bala)),
                Cell::new(&format!("{:.2}", strength.drik_bala)),
                Cell::new(&format!("{:.2}", strength.naisargika_bala)),
                Cell::new(&format!("{:.2}", strength.total)),
            ]));
        }
    }
    strength_table.printstd();

    // Vimsottari Dasha
    println!("\n{}", "Vimsottari Dasha Periods".bold());
    if let Some(moon) = planets.iter().find(|p| p.name == "Moon") {
        if let Ok(dashas) = calculate_vimsottari_dasha(moon.sidereal_long_deg, jd) {
            let mut dasha_table = Table::new();
            dasha_table.set_format(*format::consts::FORMAT_BOX_CHARS);
            dasha_table.add_row(Row::new(vec![
                Cell::new("Maha Dasha"),
                Cell::new(&format!("{} ({:.2} years)", dashas.maha_dasha.planet, dashas.maha_dasha.years)),
            ]));
            dasha_table.add_row(Row::new(vec![
                Cell::new("Antara Dasha"),
                Cell::new(&format!("{} ({:.2} years)", dashas.antara_dasha.planet, dashas.antara_dasha.years)),
            ]));
            dasha_table.add_row(Row::new(vec![
                Cell::new("Pratyantara"),
                Cell::new(&format!("{} ({:.2} years)", dashas.pratyantara_dasha.planet, dashas.pratyantara_dasha.years)),
            ]));
            dasha_table.printstd();
        }
    }

    // Yogas
    println!("\n{}", "Active Yogas".bold());
    if let Ok(yogas) = calculate_all_yogas(&planets, asc_sid_deg) {
        let mut yoga_table = Table::new();
        yoga_table.set_titles(Row::new(vec![
            Cell::new("Yoga").style_spec("b"),
            Cell::new("Strength").style_spec("b"),
            Cell::new("Description").style_spec("b"),
        ]));

        for yoga in yogas {
            yoga_table.add_row(Row::new(vec![
                Cell::new(&yoga.name),
                Cell::new(&format!("{:.2}", yoga.strength)),
                Cell::new(&yoga.description),
            ]));
        }
        yoga_table.printstd();
    }

    Ok(())
}