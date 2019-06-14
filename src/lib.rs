#[macro_use]
extern crate seed;
use seed::prelude::*;

mod asset;
mod avatar;
mod username;
mod api;
mod viewer;
mod session;
mod page;
mod article;
mod route;

// Model

enum Model {
    None,
    Redirect(session::Session),
    NotFound(session::Session),
    Home(page::home::Model),
    Settings(page::settings::Model),
    Login(page::login::Model),
    Register(page::register::Model),
    Profile(username::Username, page::profile::Model),
    Article(page::article::Model),
    ArticleEditor(Option<article::slug::Slug>, page::article_editor::Model)
}

impl Model {
    pub fn take(&mut self) -> Model {
        std::mem::replace(self, Model::None)
    }
}

impl Default for Model {
    fn default() -> Self {
        Model::None
    }
}

impl From<Model> for session::Session {
    fn from(model: Model) -> session::Session {
        match model {
            Model::None => None.into(),
            Model::Redirect(session) => session,
            Model::NotFound(session) => session,
            Model::Home(model) => model.into(),
            Model::Settings(model) => model.into(),
            Model::Login(model) => model.into(),
            Model::Register(model) => model.into(),
            Model::Profile(_, model) => model.into(),
            Model::Article(model) => model.into(),
            Model::ArticleEditor(_, model) => model.into(),
        }
    }
}

// Update

enum Msg {
    ChangedRoute(Option<route::Route>)
}

fn update(msg: Msg, model: &mut Model, _: &mut Orders<Msg>) {
    match msg {
        Msg::ChangedRoute(route) => change_route_to(route, model),
    }
}

fn change_route_to(route: Option<route::Route>, model: &mut Model) {
    match route {
        None => { *model = Model::NotFound(model.take().into()) },
        Some(route) => match route {
            route::Route::Root => {
                *model = Model::Home(page::home::init(model.take().into()))
            },
            route::Route::Logout => (),
            route::Route::NewArticle => {
                *model = Model::ArticleEditor(None, page::article_editor::init(model.take().into()))
            },
            route::Route::EditArticle(slug) => {
                *model = Model::ArticleEditor(Some(slug), page::article_editor::init(model.take().into()))
            },
            route::Route::Settings => {
                *model = Model::Settings(page::settings::init(model.take().into()))
            },
            route::Route::Home => {
                *model = Model::Home(page::home::init(model.take().into()))
            },
            route::Route::Login => {
                *model = Model::Login(page::login::init(model.take().into()))
            },
            route::Route::Register => {
                *model = Model::Register(page::register::init(model.take().into()))
            },
            route::Route::Profile(username) => {
                *model = Model::Profile(username, page::profile::init(model.take().into()))
            },
            route::Route::Article(_) => {
                *model = Model::Article(page::article::init(model.take().into()))
            },
        }
    };
}

// View

fn view(model: &Model) -> impl ElContainer<Msg> {
    let viewer = None;
    match model {
        Model::None => vec![],
        Model::Redirect(_) => page::Page::Other.view(viewer, page::blank::view()),
        Model::NotFound(_) => page::Page::Other.view(viewer, page::not_found::view()),
        Model::Settings(_) => page::Page::Settings.view(viewer,page::settings::view()),
        Model::Home(_) => page::Page::Settings.view(viewer,page::home::view()),
        Model::Login(_) => page::Page::Settings.view(viewer,page::login::view()),
        Model::Register(_) => page::Page::Settings.view(viewer,page::register::view()),
        Model::Profile(username, _) => page::Page::Profile(username).view(viewer,page::profile::view()),
        Model::Article(_) => page::Page::Other.view(viewer,page::article::view()),
        Model::ArticleEditor(None, _) => page::Page::NewArticle.view(viewer,page::article_editor::view()),
        Model::ArticleEditor(Some(_), _) => page::Page::Other.view(viewer,page::article_editor::view()),
    }
}

#[wasm_bindgen]
pub fn render() {
    seed::App::build(Model::default(), update, view)
        .routes(|url| route::url_to_msg_with_route(url, Msg::ChangedRoute))
        .finish()
        .run();
}