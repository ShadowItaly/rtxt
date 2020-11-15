use crossterm::Result;
mod screen;

fn main() -> Result<()> {
    let mut app = screen::RtxtApp::new();
    app.echo_keyboard();
    Ok(())
}
