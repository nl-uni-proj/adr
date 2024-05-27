use crate::{data, render};

pub fn analyze() {
    analyze_sign_method();
    analyze_record_values();
    corellogram();
}

#[allow(non_snake_case)]
fn analyze_sign_method() {
    let mut criteria = Vec::with_capacity(data::VALUES.len() - 1);

    for i in 0..data::VALUES.len() - 1 {
        let value = data::VALUES[i];
        let next = data::VALUES[i + 1];

        if value < next {
            criteria.push(0.0);
        } else if value == next {
            criteria.push(0.5);
        } else {
            criteria.push(1.0);
        }
    }

    let N: f64 = data::VALUES.len() as f64;
    let C: f64 = criteria.iter().sum();
    let M_C = (N - 1.0) / 2.0;
    let D_C = (N + 1.0) / 12.0;
    let K = (C - M_C) / D_C.sqrt();
    let K_ABS = K.abs();
    let normal_dist = 1.96;

    println!("\nLAB 1 Метод Знаків\n");
    println!("Число значеннь  `N`: {N}");
    println!("Сумма крітеріїв `C`: {C}");
    println!("M{{C}}               : {M_C}");
    println!("D{{C}}               : {D_C}");
    println!(" K                 : {K}");
    println!("|K|                : {K_ABS}\n");

    if K_ABS <= normal_dist {
        println!("|K| <= 1.96, ряд є випадковим");
    } else if K < -normal_dist {
        println!("K < -1.96, ряд не є випадковим, та має тенденцію спадання");
    } else if K > normal_dist {
        println!("K > 1.96, ряд не є випадковим, та має тенденцію зростання");
    }
    println!("");
}

#[allow(non_snake_case)]
fn analyze_record_values() {
    let record_lows = compute_record_lows();
    for (low, idx) in record_lows.iter().cloned() {
        println!("record low:  {low:6}, at idx: {idx:4}");
    }
    println!("");
    let record_highs = compute_record_highs();
    for (high, idx) in record_highs.iter().cloned() {
        println!("record high: {high:6}, at idx: {idx:4}");
    }

    let L = record_lows.len() as f64; // кількість нижніх рекордних значень
    let M = record_highs.len() as f64; // кількість  верхніх рекордних значень
    let D = M - L; // перший критерій Фостера–Стюарта
    let S = M + L; // другий критерій Фостера–Стюарта

    // Обчислення середнього та дисперсії для D та S
    let N = (record_lows.len() + record_highs.len()) as f64; // загальна кількість спостережень
    let E_D = 0.0;
    let E_S = (N - 1.0) / 2.0; // середнє для S

    // Дисперсія для D та S
    let var_D = 2.0 / (N - 2.0);
    let var_S = 4.0 * (N - 2.0) / (N * (N - 1.0));

    let sigma_D = var_D.sqrt();
    let sigma_S = var_S.sqrt();

    // Обчислення T1 та T2
    let T1 = (D - E_D) / sigma_D;
    let T2 = (S - E_S) / sigma_S;

    println!("\nLAB 1 Метод Рекордних Значень\n");
    println!("L: {L:4} - кількість нижніх рекордних значень");
    println!("M: {M:4} - кількість верхніх рекордних значень");
    println!("D: {D:4} - перший критерій Фостера–Стюарта");
    println!("S: {S:4} - другий критерій Фостера–Стюарта");
    println!("T1: {T1:.4} - статистична характеристика для D");
    println!("T2: {T2:.4} - статистична характеристика для S");

    println!("\nТестування гіпотез");
    let u_alpha = 1.96; // квантиль нормального розподілу для 95% рівня значущості

    if T1.abs() <= u_alpha {
        println!("Основна гіпотеза для D приймається: немає тренду середнього рівня.");
    } else if T1 > u_alpha {
        println!(
            "Відхиляємо основну гіпотезу для D: існує тенденція до зростання середнього рівня."
        );
    } else {
        println!(
            "Відхиляємо основну гіпотезу для D: існує тенденція до спадання середнього рівня."
        );
    }

    if T2.abs() <= u_alpha {
        println!("Основна гіпотеза для S приймається: немає тенденції зміни дисперсії.");
    } else if T2 > u_alpha {
        println!("Відхиляємо основну гіпотезу для S: існує тенденція до зростання дисперсії.");
    } else {
        println!("Відхиляємо основну гіпотезу для S: існує тенденція до спадання дисперсії.");
    }
    println!("");
}

fn compute_record_lows() -> Vec<(f64, usize)> {
    let mut record_lows = Vec::<(f64, usize)>::new();
    let mut min_low = f64::MIN;

    for i in 0..data::VALUES.len() - 1 {
        let value = data::VALUES[i];
        let next = data::VALUES[i + 1];
        if next >= value {
            min_low = value;
            record_lows.push((min_low, i));
            break;
        }
    }
    for i in 0..data::VALUES.len() - 1 {
        let value = data::VALUES[i];
        let next = data::VALUES[i + 1];
        if next >= value {
            if value < min_low {
                min_low = value;
                record_lows.push((min_low, i));
            }
        }
    }

    record_lows
}

fn compute_record_highs() -> Vec<(f64, usize)> {
    let mut record_highs = Vec::<(f64, usize)>::new();
    let mut max_high = f64::MIN;

    for i in 0..data::VALUES.len() - 1 {
        let value = data::VALUES[i];
        let next = data::VALUES[i + 1];
        if next <= value {
            if value > max_high {
                max_high = value;
                record_highs.push((max_high, i));
            }
        }
    }

    record_highs
}

use statrs::statistics::Statistics;

fn corellogram() {
    let mut acf_values = compute_acf(data::VALUES, 365);
    for v in acf_values.iter_mut() {
        *v = *v * 100.0;
    }
    render::render(
        acf_values.as_slice(),
        Some("lab1"),
        "correlogram",
        "Correlogram",
    );
}

fn compute_acf(data: &[f64], max_lag: usize) -> Vec<f64> {
    let n = data.len();
    let mean = data.mean();
    let var = data.variance();

    (0..=max_lag)
        .map(|lag| {
            let cov: f64 = (0..n - lag)
                .map(|i| (data[i] - mean) * (data[i + lag] - mean))
                .sum();
            cov / (var * (n - lag) as f64)
        })
        .collect()
}
