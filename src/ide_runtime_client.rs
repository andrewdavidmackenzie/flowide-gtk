use flowrlib::client_server::RuntimeClientConnection;
use flowrlib::coordinator::Submission;
use flowrlib::runtime::{Event, Response};
use flowrlib::runtime::Response::ClientSubmission;
use std::io::Write;
// use gtk::{WidgetExt, ApplicationWindow, WindowPosition};
// use gtk::prelude::*;
use gtk::TextBufferExt;

use crate::build_ui::widgets;
use crate::ui_context::UiContext;
use std::fs::File;

#[derive(Debug, Clone)]
pub struct IdeRuntimeClient {
    args: Vec<String>,
    display_metrics: bool,
}

impl IdeRuntimeClient {
    // Create a new Runtime Client - an IDE version
    fn new(args: Vec<String>, display_metrics: bool) -> Self {
        IdeRuntimeClient {
            args,
            display_metrics,
        }
    }

    /// Enter a client for runtime that runs in a loop receiving events and responding to them
    pub fn start(mut connection: RuntimeClientConnection,
                 submission: Submission, flow_args: Vec<String>) {
        if let Err(e) = connection.start() {
            UiContext::ui_error(&format!("Error while starting IDE Runtime client, creating connection: {}", e));
        }

        if let Err(e) = connection.client_send(ClientSubmission(submission)) {
            UiContext::ui_error(&format!("Error while starting IDE Runtime client, client_send: {}", e));
        }

        let mut runtime_client = Self::new(flow_args, true /* display_metrics */);

        loop {
            match connection.client_recv() {
                Ok(event) => {
                    let response = runtime_client.process_event(event);
                    if response == Response::ClientExiting {
                        UiContext::message("Flow execution ended");
                        return;
                    }

                    let _ = connection.client_send(response);
                }
                Err(e) => UiContext::ui_error(&format!("Error receiving Event in runtime client: {}", e))
            }
        }
    }

    fn process_event(&mut self, event: Event) -> Response {
        match event {
            Event::FlowStart => Response::Ack,
            Event::FlowEnd(_metrics) => Response::ClientExiting,
            Event::StdoutEOF => Response::Ack,
            Event::Stdout(contents) => {
                widgets::do_in_gtk_eventloop(|refs| {
                    refs.stdout().insert_at_cursor(&format!("{}\n", contents));
                });
                Response::Ack
            }
            Event::Stderr(contents) => {
                widgets::do_in_gtk_eventloop(|refs| {
                    refs.stderr().insert_at_cursor(&format!("{}\n", contents));
                });
                Response::Ack
            }
            Event::GetStdin => {
                Response::Stdin("bla bla".to_string()) // TODO
            }
            Event::GetLine => {
                Response::Stdin("bla bla".to_string())  // TODO
            }
            Event::GetArgs => {
                Response::Args(self.args.clone())
            }
            Event::Write(filename, bytes) => {
                let mut file = File::create(filename).unwrap();
                file.write_all(bytes.as_slice()).unwrap();
                Response::Ack
            }
            Event::PixelWrite((_x, _y), (_r, _g, _b), (_width, _height), _name) => {
                // widgets::do_in_gtk_eventloop(|refs| {
                //     let pix_buf = refs.image();
                //     if pix_buf.get_width() == 0 {
                //         pix_buf.set_width(width as i32);
                //         pix_buf.set_height(height as i32);
                //         let image = gtk::Image::from_pixbuf(Some(&pix_buf));
                //
                //         let image_window = ApplicationWindow::new(&refs.app());
                //         image_window.set_title(&name);
                //         image_window.set_position(WindowPosition::Center);
                //         image_window.set_size_request(width as i32, height as i32);
                //         image_window.add(&image);
                //         image_window.show_all();
                //     }
                //     pix_buf.put_pixel(x, y, r, g, b, 255);
                // });

                Response::Ack
            }
            Event::StderrEOF => Response::Ack,
            Event::Invalid => Response::Ack,
        }
    }
}