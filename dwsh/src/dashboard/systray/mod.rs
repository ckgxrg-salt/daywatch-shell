//! The system tray

mod item;

use gtk4::prelude::*;
use relm4::prelude::*;

use futures::StreamExt;
use std::sync::Arc;
use wayle_systray::core::item::TrayItem;

use crate::services::tray_service;
use item::SystrayItem;

pub struct Systray {
    items: FactoryVecDeque<SystrayItem>,
}

#[derive(Debug)]
pub enum TrayCmd {
    UpdateItems(Vec<Arc<TrayItem>>),
}

#[relm4::component(pub)]
impl Component for Systray {
    type Init = ();
    type Input = ();
    type Output = ();
    type CommandOutput = TrayCmd;

    view! {
        gtk::Box {
            add_css_class: "tray",

            #[local_ref]
            items_widget -> gtk::Box {},
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        watch_tray(&sender);

        let items = FactoryVecDeque::builder()
            .launch(gtk::Box::default())
            .detach();

        let model = Self { items };

        let items_widget = model.items.widget();
        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update_cmd(
        &mut self,
        message: Self::CommandOutput,
        _sender: ComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            TrayCmd::UpdateItems(value) => {
                let mut guard = self.items.guard();
                guard.clear();
                value.iter().for_each(|item| {
                    guard.push_back(item.clone());
                });
            }
        }
    }
}

fn watch_tray(sender: &ComponentSender<Systray>) {
    let service = tray_service();
    let mut stream = service.items.watch();

    sender.command(|out, shutdown| {
        shutdown
            .register(async move {
                while let Some(value) = stream.next().await {
                    let _ = out.send(TrayCmd::UpdateItems(value));
                }
            })
            .drop_on_shutdown()
    });
}
