use actix_session::UserSession;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::header::LOCATION,
    Error, HttpResponse,
};
use futures::future::ok;
use futures::future::{Either, Ready};

pub struct LoginRequired;
impl<S, B> Transform<S> for LoginRequired
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Transform = LoginRequiredMiddleware<S>;

    type InitError = ();

    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoginRequiredMiddleware { service })
    }
}
pub struct LoginRequiredMiddleware<S> {
    service: S,
}

impl<S, B> Service for LoginRequiredMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
{
    type Request = ServiceRequest;

    type Response = ServiceResponse<B>;

    type Error = Error;

    type Future = Either<S::Future, Ready<Result<Self::Response, Self::Error>>>;

    fn poll_ready(
        &mut self,
        ctx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&mut self, req: Self::Request) -> Self::Future {
        // check if the user is logged in
        let session = req.get_session();
        let is_logged_in: bool = session.get("login").unwrap().unwrap_or(false);

        if is_logged_in || req.path() == "/login" {
            Either::Left(self.service.call(req))
        } else {
            Either::Right(ok(req.into_response(
                HttpResponse::Found()
                    .set_header(LOCATION, "/login")
                    .finish()
                    .into_body(),
            )))
        }
    }
}
