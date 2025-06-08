use crate::{ ctx::Ctx, Error, Result };
use axum::{ Json, Router, extract::{ Path, State }, routing::{ delete, get, post } };

use crate::model::{ ModelController, Ticket, TicketForCreate };

/* -- Exmaple of passing multiple states / entire application states
#[Derive(Clone, FromRef)] // for FromRef, we have to add feature : macros in axum
struct AppState {
    mc : ModelController,
    st1 : StateOne,
    st2 : StateTwo
    ....

}

*/

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        /* Two diff methods for same  endpoint */
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/ticket/{id}", delete(delete_ticket))
        .with_state(mc) // to pass state to all handlers
}

// region: -- Rest Handlers

async fn create_ticket(
    State(mc): State<ModelController> /* way to pass and extract application level states */,
    ctx: Ctx,
    Json(ticket_fc): Json<TicketForCreate>
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    let ticket = mc.create_ticket(ctx, ticket_fc).await?;
    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");
    let tickets = mc.list_tickets(ctx).await?;

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>
) -> Result<Json<Ticket>> {
    println!("->> {:<15} - delete_ticket", "HANDLER");
    let ticket = mc.delete_ticket(ctx, id).await?;

    Ok(Json(ticket))
}

// endregion: -- Rest Handlers
