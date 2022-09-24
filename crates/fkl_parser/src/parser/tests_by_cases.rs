#[cfg(test)]
mod test {
  use crate::{mir, parse};
  use crate::mir::{BoundedContext, ContextRelation, ContextState};
  use crate::mir::ConnectionDirection::PositiveDirected;

  #[test]
  fn test() {
    let booking_ticket = r#"
ContextMap TicketBooking {
  Reservation -> Cinema;
  Reservation -> Movie;
  Reservation -> User;
}

Aggregate Reservation {
  Entity Ticket, Reservation;
}

Entity Reservation  {
  Struct {
    id: String;
    token: UUID;
    status: ReservationStatus = ReservationStatus.OPEN;
    expiresAt: LocalDateTime;
    createdAt: LocalDateTime;
    screeningId: String;
    screeningStartTime: LocalDateTime;
    name: String;
    surname: String;
    tickets: Set<Ticket>;
    totalPrice: BigDecimal;
  }
}

Entity Ticket  {}

Aggregate Cinema {
  Entity Cinema, ScreeningRoom, Seat;
}

Entity Cinema { }
Entity ScreeningRoom { }
Entity Seat { }

Aggregate Movie {
  Entity Movie, Actor, Publisher;
}

Entity Movie { }
Entity Actor { }
Entity Publisher { }

Aggregate User {
  Entity User;
}

Entity User {
  Struct {
    id: UUID;
    mobile: String;
    email: String;
    username: String;
    password: String;
    address: String;
  }
}

ValueObject Payment {
  Struct {
    id: UUID;
    amount: BigDecimal;
    currency: Currency;
    status: PaymentStatus;
    createdAt: LocalDateTime;
  }
}

ValueObject Price { }
ValueObject Notifications { }
"#;

    let decls = parse(booking_ticket).unwrap();
    assert_eq!(decls, mir::ContextMap {
      name: "".to_string(),
      state: ContextState::ToBe,
      contexts: vec![
        BoundedContext { name: "Cinema".to_string() },
        BoundedContext { name: "Movie".to_string() },
        BoundedContext { name: "Reservation".to_string() },
        BoundedContext { name: "User".to_string() }],
      relations: vec![
        ContextRelation {
          source: "Reservation".to_string(),
          target: "Cinema".to_string(),
          connection_type: PositiveDirected,
          source_type: vec![],
          target_type: vec![],
        },
        ContextRelation {
          source: "Reservation".to_string(),
          target: "Movie".to_string(),
          connection_type: PositiveDirected,
          source_type: vec![],
          target_type: vec![],
        }
        ,
        ContextRelation {
          source: "Reservation".to_string(),
          target: "User".to_string(),
          connection_type: PositiveDirected,
          source_type: vec![],
          target_type: vec![],
        }],
    });
  }
}
