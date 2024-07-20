#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use rand::Rng;
use rdev::{grab, simulate, Button, Event, EventType};
use tray_item::{IconSource, TrayItem};

static ACTIVE: AtomicBool = AtomicBool::new(true);
static CLICKING: AtomicBool = AtomicBool::new(false);
static TOGGLE_MODE: AtomicBool = AtomicBool::new(false);

enum Message {
    ToggleMode,
    ToggleActive,
    Quit,
    UpdateTray,
}

fn main() {
    let mut tray = TrayItem::new("acric", IconSource::Resource("app-icon")).unwrap();
    let (tx, rx) = mpsc::sync_channel(1);

    tray.inner_mut().add_label("acric by z1xus <3").unwrap();
    tray.inner_mut().add_separator().unwrap();

    let active_toggle_tx = tx.clone();
    let active_toggle_id = tray
        .inner_mut()
        .add_menu_item_with_id("Active (On) [F6]", move || {
            active_toggle_tx.send(Message::ToggleActive).unwrap();
        })
        .unwrap();

    let toggle_mode_tx = tx.clone();
    let toggle_mode_id = tray
        .inner_mut()
        .add_menu_item_with_id("Hold to Click [MB5]", move || {
            toggle_mode_tx.send(Message::ToggleMode).unwrap();
        })
        .unwrap();

    let quit_tx = tx.clone();
    tray.add_menu_item("Exit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    let event_tx = tx.clone();
    std::thread::spawn(move || {
        let _ = grab(move |event| event_handler(event, &event_tx));
    });

    std::thread::spawn(clicker_thread);

    loop {
        match rx.recv() {
            Ok(Message::ToggleMode) => {
                let new_state = !TOGGLE_MODE.fetch_xor(true, Ordering::Relaxed);
                let new_label = if new_state {
                    "Toggle to Click [MB5]"
                } else {
                    "Hold to Click [MB5]"
                };
                tray.inner_mut()
                    .set_menu_item_label(new_label, toggle_mode_id)
                    .unwrap();
            }
            Ok(Message::ToggleActive) => {
                let new_state = !ACTIVE.load(Ordering::Relaxed);
                ACTIVE.store(new_state, Ordering::Relaxed);
                update_active_state(&mut tray, active_toggle_id, new_state);
            }
            Ok(Message::UpdateTray) => {
                let active = ACTIVE.load(Ordering::Relaxed);
                update_active_state(&mut tray, active_toggle_id, active);
            }
            Ok(Message::Quit) => {
                std::process::exit(0);
            }
            _ => {}
        }
    }
}

fn event_handler(event: Event, tx: &mpsc::SyncSender<Message>) -> Option<Event> {
    match event.event_type {
        EventType::KeyPress(rdev::Key::F6) => {
            let _new_state = !ACTIVE.fetch_xor(true, Ordering::Relaxed);
            tx.send(Message::UpdateTray).unwrap();
            #[cfg(debug_assertions)]
            println!("f6 pressed, new active state: {}", _new_state);
            None
        }
        EventType::ButtonPress(Button::Unknown(1)) if ACTIVE.load(Ordering::Relaxed) => {
            hotkey_press();
            None
        }
        EventType::ButtonRelease(Button::Unknown(1)) if ACTIVE.load(Ordering::Relaxed) => {
            hotkey_release();
            None
        }
        _ => Some(event),
    }
}

fn update_active_state(tray: &mut TrayItem, active_toggle_id: u32, active: bool) {
    let new_label = if active {
        "Active (On) [F6]"
    } else {
        "Active (Off) [F6]"
    };
    tray.inner_mut()
        .set_menu_item_label(new_label, active_toggle_id)
        .unwrap();
    update_tray(tray, active);
}

fn hotkey_press() {
    if TOGGLE_MODE.load(Ordering::Relaxed) {
        CLICKING.fetch_xor(true, Ordering::Relaxed);
    } else {
        CLICKING.store(true, Ordering::Relaxed);
    }
    #[cfg(debug_assertions)]
    println!(
        "clicking {}",
        if CLICKING.load(Ordering::Relaxed) {
            "started"
        } else {
            "stopped"
        }
    );
}

fn hotkey_release() {
    if !TOGGLE_MODE.load(Ordering::Relaxed) {
        CLICKING.store(false, Ordering::Relaxed);
        #[cfg(debug_assertions)]
        println!("clicking stopped");
    }
}

fn update_tray(tray: &mut TrayItem, active: bool) {
    let icon = if active {
        IconSource::Resource("app-icon")
    } else {
        IconSource::Resource("app-icon-inactive")
    };
    tray.set_icon(icon).unwrap();
}

fn calculate_cps(rng: &mut impl Rng, t: f64, fatigue: f64) -> f64 {
    let base_cps = 13.0 * (1.0 - f64::exp(-0.5 * t));

    let time_factor = (t * 0.1).sin().abs() + 0.5;
    let amplitude = 2.0 + rng.gen_range(0.0..1.0);
    let sin_variation = (t * 2.0).sin() * amplitude * time_factor;

    let noise = rng.gen_range(-0.5..0.5);

    let mut cps = base_cps + sin_variation + noise;

    if rng.gen_bool(0.02) {
        cps += rng.gen_range(0.5..1.5);
    }

    cps *= 1.0 - (fatigue * 0.3);

    cps.clamp(0.1, 18.0)
}

fn click(rng: &mut impl Rng) {
    let delay = Duration::from_millis(rng.gen_range(5..=25));
    let _ = simulate(&EventType::ButtonPress(Button::Left));
    std::thread::sleep(delay);
    let _ = simulate(&EventType::ButtonRelease(Button::Left));
    std::thread::sleep(delay);
}

fn clicker_thread() {
    let mut rng = rand::thread_rng();
    let mut t = 0.0;
    let mut last_click = Instant::now();
    let mut click_count = 0;
    let mut session_start = Instant::now();

    loop {
        if CLICKING.load(Ordering::Relaxed) && ACTIVE.load(Ordering::Relaxed) {
            let now = Instant::now();
            t += now.duration_since(last_click).as_secs_f64();

            let fatigue = (now.duration_since(session_start).as_secs_f64() / 60.0).min(1.0);

            let cps = calculate_cps(&mut rng, t, fatigue);
            let click_interval = if cps > 0.0 {
                Duration::from_secs_f64(1.0 / cps)
            } else {
                Duration::from_secs(1)
            };

            let variable_interval = click_interval + Duration::from_millis(rng.gen_range(0..5));

            if now.duration_since(last_click) >= variable_interval {
                if rng.gen_bool(0.99) {
                    click(&mut rng);
                } else if rng.gen_bool(0.5) {
                    #[cfg(debug_assertions)]
                    println!("misclick!");
                } else {
                    click(&mut rng);
                    std::thread::sleep(Duration::from_millis(rng.gen_range(10..30)));
                    click(&mut rng);
                    #[cfg(debug_assertions)]
                    println!("double click!");
                }

                last_click = now;
                click_count += 1;

                #[cfg(debug_assertions)]
                println!("clicked at {:.2} cps (fatigue: {:.2})", cps, fatigue);

                if rng.gen_bool(0.1) {
                    let pause_duration = Duration::from_millis(rng.gen_range(20..50));
                    std::thread::sleep(pause_duration);
                    #[cfg(debug_assertions)]
                    println!("short pause for {:?}", pause_duration);
                }

                if click_count > 50 && rng.gen_bool(0.02) {
                    let pause_duration = Duration::from_millis(rng.gen_range(100..300));
                    std::thread::sleep(pause_duration);
                    click_count = 0;
                    t = 0.0;
                    #[cfg(debug_assertions)]
                    println!("long pause for {:?}", pause_duration);
                }
            }
        } else {
            t = 0.0;
            last_click = Instant::now();
            click_count = 0;
            session_start = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(1));
    }
}
