mod affinities;
mod code_now;
mod languages;
mod not_found;
mod services;
mod settings;

use self::affinities::AffinitiesPage;
use self::code_now::CodeNowPage;
use self::not_found::NotFoundPage;
use self::services::UserService;
use self::settings::SettingsPage;

use yew::prelude::*;
use yew::virtual_dom::VNode;
use yew_router::switch::Permissive;
use yew_router::{prelude::*, Switch};

use devand_core::User;

type RouterAnchor = yew_router::components::RouterAnchor<AppRoute>;

#[derive(Switch, Debug, Clone)]
pub enum AppRoute {
    #[to = "/affinities"]
    Affinities,
    #[to = "/code-now"]
    CodeNow,
    #[to = "/page-not-found"]
    NotFound(Permissive<String>),
    #[to = "/dashboard"]
    Settings,
}

pub struct App {
    user_service: UserService,
    state: State,
    link: ComponentLink<Self>,
}

#[derive(Default)]
pub struct State {
    user: Option<User>,
    pending_save: bool,
}

pub enum Msg {
    UserStore(User),
    UserFetchOk(User),
    UserFetchErr,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let fetch_callback = link.callback(|result: Result<User, anyhow::Error>| match result {
            Ok(user) => Msg::UserFetchOk(user),
            Err(err) => {
                log::error!("{:?}", err);
                Msg::UserFetchErr
            }
        });

        let mut user_service = UserService::new(fetch_callback);
        user_service.restore();

        App {
            user_service,
            state: State::default(),
            link,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UserStore(user) => {
                self.user_service.store(&user);
                false
            }
            Msg::UserFetchOk(user) => {
                log::debug!("User fetch ok");
                self.state.user = Some(user);
                self.state.pending_save = false;
                true
            }
            Msg::UserFetchErr => {
                log::error!("User fetch error");
                false
            }
        }
    }

    fn view(&self) -> VNode {
        let on_settings_change = self.link.callback(Msg::UserStore);
        let user = self.state.user.clone();
        html! {
            <>
            <div>
                <RouterAnchor route=AppRoute::Settings classes="pure-button" >{ "Settings" }</RouterAnchor>
                <RouterAnchor route=AppRoute::Affinities classes="pure-button" >{ "Affinities" }</RouterAnchor>
                <RouterAnchor route=AppRoute::CodeNow classes="pure-button" >{ "Code Now" }</RouterAnchor>
            </div>
            <Router<AppRoute>
                render = Router::render(move |switch: AppRoute| {
                    match switch {
                        AppRoute::Settings=> html!{ <SettingsPage on_change=on_settings_change.clone() user=user.clone() /> },
                        AppRoute::Affinities=> html!{ <AffinitiesPage/> },
                        AppRoute::CodeNow=> html!{ <CodeNowPage/> },
                        AppRoute::NotFound(Permissive(missed_route)) => html!{ <NotFoundPage missed_route=missed_route/>},
                        _ => todo!()
                    }
                })
                redirect = Router::redirect(|route: Route| { AppRoute::NotFound(Permissive(Some(route.route))) })
            />
            </>
        }
    }
}
