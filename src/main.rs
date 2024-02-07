use financial;
use thousands::Separable;
slint::include_modules!();

const TAX_CLAW: f64 = 0.30;
const NEEDS: f64 = 0.35;
const WANTS: f64 = 0.105;
const INVEST: f64 = 0.245;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    
    ui.on_calculate_salary({
        let ui_handle = ui.as_weak();
        move |salary, initial, rate, years| {
            let ui = ui_handle.unwrap();
            let num: f64 = salary.trim().parse().unwrap();
            let tax: f64 = num * TAX_CLAW;
            let need: f64 = num * NEEDS;
            let want: f64 = num * WANTS;
            let invest: f64 = num * INVEST; // AMOUNT TO INVEST

            let initial_value: f64 = initial.trim().parse().unwrap(); // INITIAL VALUE
            let rate_percent: f64 = rate.trim().parse().unwrap(); // RATE WHOLE NUMBER
            let time_horizon: f64 = years.trim().parse().unwrap(); // YEARS INVESTED
            
            let fv = financial::fv(rate_percent / 100.0, time_horizon, Some(invest), Some(initial_value), Some(true));

            let result = format!("Taxes: {:.2}\nNeeds: {:.2}\nWants: {:.2}\n Invest: {:.2}\n\n Future Value: ${:.2}", tax, need, want, invest, -fv);
            ui.set_results(result.into());
        }
    });
    

    ui.run()
}


// fn calculate_future_value(initial_value: f64, invest: f64, rate_percent: f64, time_horizon: f64) -> f64 {

//     let months_invested = time_horizon as f64 * 12.0;
//     let monthly_return_rate = rate_percent / 12.0 / 100.0; // CONVERT TO DECIMAL AND COMPOUND MONTHLY
//     let future_value = initial_value * (1.0 + monthly_return_rate).powf(months_invested) + invest * ((1.0 + monthly_return_rate).powf(months_invested) - 1.0) / monthly_return_rate;

//     future_value
// }

// let future_value = calculate_future_value(initial_value, invest, rate_percent, time_horizon);