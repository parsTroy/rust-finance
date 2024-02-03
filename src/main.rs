slint::include_modules!();

const TAX_CLAW: f64 = 0.30;
const NEEDS: f64 = 0.35;
const WANTS: f64 = 0.105;
const INVEST: f64 = 0.245;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_calculate_salary({
        let ui_handle = ui.as_weak();
        move |string| {
            let ui = ui_handle.unwrap();
            let num: f64 = string.trim().parse().unwrap();
            let tax: f64 = num * TAX_CLAW;
            let need: f64 = num * NEEDS;
            let want: f64 = num * WANTS;
            let invest: f64 = num * INVEST;
            let result = format!("Taxes: {:.2}\nNeeds: {:.2}\nWants: {:.2}\n Invest: {:.2}", tax, need, want, invest);
            ui.set_results(result.into());
        }
    });

    ui.run()
}