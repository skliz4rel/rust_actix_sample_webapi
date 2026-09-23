use utoipa::OpenApi;

use utoipa_swagger_ui::{SwaggerUi, Config};

use utoipa::{
    Modify,
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
};

use crate::handlers::booking_handler::{cancel_booking,get_bookings, create_booking};


use crate::models::{booking_model::{BookingRequest,FullBooking}, dog_model::DogRequest, owner_model::OwnerRequest};

#[derive(OpenApi)]
#[openapi(
    paths(
       crate::handlers::health_check_handler::health_check,
        crate::handlers::booking_handler::cancel_booking,
crate::handlers::booking_handler::get_bookings, 
crate::handlers::booking_handler::create_booking,
        crate::handlers::dog_handler::create_dog,
   crate::handlers::owner_handler::create_owner
        
    ),
    components(
        schemas(
            DogRequest,
            BookingRequest,
            FullBooking,
           OwnerRequest          
        )
    ),
    tags(
     (name = "Health", description = "Health endpoints"),
        (name = "Bookings", description = "Bookings Endpoint"),
		(name = "Owners", description = "Owners Endpoint"),
		(name = "Dogs", description = "Dogs Endpoint"),
    )
)]
pub struct ApiDoc;
