// use std::fs::File;
// use std::io::prelude::*;
//
// use gtk::TextBufferExt;

// use flowrlib::runtime::{Event, Response};
// use flowrlib::lib.client_server::RuntimeClientConnection;
use std::collections::HashMap;
use crate::{widgets};
use crate::UIContext;
use image::{ImageBuffer, Rgb};
use flowrlib::client_server::RuntimeClientConnection;
use flowrlib::coordinator::Submission;
use flowrlib::runtime::{Event, Response};
use std::fs::File;
use flowrlib::runtime::Response::ClientSubmission;
use std::io::Write;
use gtk::TextBufferExt;

#[derive(Debug, Clone)]
pub struct IDERuntimeClient {
    args: Vec<String>,
    image_buffers: HashMap<String, ImageBuffer<Rgb<u8>, Vec<u8>>>,
    display_metrics: bool,
}

impl IDERuntimeClient {
    pub fn new(args: Vec<String>, display_metrics: bool) -> Self {
        IDERuntimeClient {
            args,
            image_buffers: HashMap::<String, ImageBuffer<Rgb<u8>, Vec<u8>>>::new(),
            display_metrics,
        }
    }

    /*
        Enter  a loop where we receive events as a client and respond to them
     */
    pub fn start(mut connection: RuntimeClientConnection,
                 submission: Submission, flow_args: Vec<String>) {
        if let Err(e) = connection.start() {
            UIContext::ui_error(&format!("Error while starting IDE Runtime client, creating connection: {}", e));
        }

        if let Err(e) = connection.client_send(ClientSubmission(submission)) {
            UIContext::ui_error(&format!("Error while starting IDE Runtime client, client_send: {}", e));
        }

        let mut runtime_client = Self::new(flow_args, true /* display_metrics */);

        loop {
            match connection.client_recv() {
                Ok(event) => {
                    let response = runtime_client.process_event(event);
                    if response == Response::ClientExiting {
                        UIContext::message("Flow execution ended");
                        return;
                    }

                    let _ = connection.client_send(response);
                }
                Err(e) => UIContext::ui_error(&format!("Error receiving Event in runtime client: {}", e))
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
//                Response::Stdin("bla bla".to_string()) // TODO
                Response::Error("Could not read Stdin".into())
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
                // let image = self.image_buffers.entry(name)
                //     .or_insert(RgbImage::new(width, height));
                // image.put_pixel(x, y, Rgb([r, g, b]));
                Response::Ack
            }
            Event::StderrEOF => Response::Ack,
            Event::Invalid => Response::Ack
        }
    }
}