use std::io::Write;

mod gpio;
mod http;

struct LightHandler {
    pin: gpio::OutputPin,
}

impl http::Handler for LightHandler {
    fn handle(&self, r: &http::Request, w: &mut http::ResponseWriter) {
        if r.method() != "PUT" {
            w.set_status(405);
            return;
        }

        match r.path().split('/').last() {
            Some("on") => {
                self.pin.set_value(gpio::Value::Low).unwrap();
                write!(w.body(), "ok").unwrap();
            }
            Some("off") => {
                self.pin.set_value(gpio::Value::High).unwrap();
                write!(w.body(), "ok").unwrap();
            }
            _ => {
                w.set_status(404);
            }
        }
    }
}

fn main() {
    let pin = gpio::OutputPin::new(17).unwrap();
    let handler = LightHandler { pin };

    let mut s = http::Server::new(8080);
    s.register("/light/on", &handler);
    s.register("/light/off", &handler);
    s.serve();
}
