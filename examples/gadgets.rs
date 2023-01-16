#![no_std]
#![no_main]

use core::panic::PanicInfo;
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

use nanos_sdk::buttons::*;
use nanos_ui::layout::{Layout, Location, StringPlace};
use nanos_ui::ui;

fn wait_any() {
    let mut buttons = ButtonsState::new();
    loop {
        match ui::get_event(&mut buttons) {
            Some(ButtonEvent::LeftButtonRelease)
            | Some(ButtonEvent::RightButtonRelease)
            | Some(ButtonEvent::BothButtonsRelease) => return,
            _ => (),
        }
    }
}

#[no_mangle]
extern "C" fn sample_main() {
    ui::clear_screen();
    ui::popup("Hello");

    ui::clear_screen();

    /*["0x07e00d496e324876","bbc8531f2d9a82bf", "154d1a04a50218ee", "74cdd372f75a551a"].place(Location::Middle, Layout::Centered, false);
    wait_any();
    ui::clear_screen();

    ["First Line", "Second Line", "Third Line"].place(Location::Middle, Layout::Centered, false);
    wait_any();
    ui::clear_screen();

    ["First Line", "Second Line", "Third Line", "Fourth", "Fifth", "Sixth"].place(
        Location::Middle,
        Layout::Centered,
        true,
    );
    wait_any();
    ui::clear_screen();

    ["Monero &", "Ethereum &", "Zcash &", "NanoPass"].place(
        Location::Top,
        Layout::LeftAligned,
        false,
    );
    wait_any();
    ui::clear_screen();

    ["Monero &", "Ethereum &", "Zcash &", "NanoPass"].place(
        Location::Top,
        Layout::RightAligned,
        false,
    );
    wait_any();

    let scrolled_message = "Arbitrary long text goes here, with numbers -1234567890";
    ui::MessageScroller::new(scrolled_message).event_loop();

    loop {
        match ui::Menu::new(&[&"Top0", &"Top1", &"Top2", &"Top3", &"Next"]).show() {
            0 => loop {
                match ui::Menu::new(&[&"Top0_sub0", &"Back"]).show() {
                    0 => ui::popup("Top0_sub0_0"),
                    _ => break,
                }
            },
            1 => loop {
                match ui::Menu::new(&[&"Top1_sub0", &"Top1_sub1", &"Back"]).show() {
                    0 => ui::popup("Top1_sub0_0"),
                    1 => ui::popup("Top1_sub1_0"),
                    _ => break,
                }
            },
            2 => break,
            3 => break,
            4 => break,
            _ => (),
        }
    }

    let _ = ui::Validator::new("Confirm?").ask();
    */
    let _ = ui::MessageValidator::new(
        &[&"Account:", &"0x07e00d496e324876",&"bbc8531f2d9a82bf", &"154d1a04a50218ee", &"74cdd372f75a551a"],
        &[&"Confirm"],
        &[&"Cancel"],
    )
    .ask();

    ui::clear_screen();

    use nanos_ui::bagls::RectFull as Rect;
    use nanos_ui::layout::Draw;

    Rect::new().width(10).height(10).pos(16, 16).display();
    Rect::new().width(10).height(10).pos(32, 16).display();
    Rect::new().width(10).height(10).pos(48, 16).display();

    wait_any();
    nanos_sdk::exit_app(0);
}
