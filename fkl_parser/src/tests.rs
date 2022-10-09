#[cfg(test)]
mod test {
  use crate::mir;
  use crate::mir::{Aggregate, BoundedContext, ContextRelation, ContextState, Entity};
  use crate::mir::ConnectionDirection::PositiveDirected;
  use crate::mir::tactic::block::Field;
  use crate::parse;

  #[test]
  fn test() {
    let booking_ticket = r#"
ContextMap TicketBooking {
  Reservation -> Cinema;
  Reservation -> Movie;
  Reservation -> User;
}

Context Reservation {
  Aggregate Reservation;
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

Context Cinema {
  Aggregate Cinema;
}

Aggregate Cinema {
  Entity Cinema, ScreeningRoom, Seat;
}

Entity Cinema { }
Entity ScreeningRoom { }
Entity Seat { }

Context Movie {
  Aggregate Movie;
}

Aggregate Movie {
  Entity Movie, Actor, Publisher;
}

Entity Movie { }
Entity Actor { }
Entity Publisher { }

Context User {
  Aggregate User;
}

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

Entity Payment {
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
      name: "TicketBooking".to_string(),
      state: ContextState::ToBe,
      contexts: vec![
        BoundedContext {
          name: "Cinema".to_string(),
          aggregates: vec![
            Aggregate {
              name: "Cinema".to_string(),
              description: "".to_string(),
              entities: vec![
                Entity {
                  name: "Cinema".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field { name: "".to_string(), initializer: None, type_type: "".to_string() },
                  fields: vec![],
                },
                Entity {
                  name: "ScreeningRoom".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field { name: "".to_string(), initializer: None, type_type: "".to_string() },
                  fields: vec![],
                },
                Entity {
                  name: "Seat".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field {
                    name: "".to_string(),
                    initializer: None,
                    type_type: "".to_string(),
                  },
                  fields: vec![],
                },
              ],
            }
          ],
        },
        BoundedContext {
          name: "Movie".to_string(),
          aggregates: vec![
            Aggregate {
              name: "Movie".to_string(),
              description: "".to_string(),
              entities: vec![
                Entity {
                  name: "Movie".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field {
                    name: "".to_string(),
                    initializer: None,
                    type_type: "".to_string(),
                  },
                  fields: vec![],
                },
                Entity {
                  name: "Actor".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field {
                    name: "".to_string(),
                    initializer: None,
                    type_type: "".to_string(),
                  },
                  fields: vec![],
                },
                Entity {
                  name: "Publisher".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field {
                    name: "".to_string(),
                    initializer: None,
                    type_type: "".to_string(),
                  },
                  fields: vec![],
                },
              ],
            }
          ],
        },
        BoundedContext {
          name: "Reservation".to_string(),
          aggregates: vec![
            Aggregate {
              name: "Reservation".to_string(),
              description: "".to_string(),
              entities: vec![
                Entity {
                  name: "Ticket".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field {
                    name: "".to_string(),
                    initializer: None,
                    type_type: "".to_string(),
                  },
                  fields: vec![],
                },
                Entity {
                  name: "Reservation".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field {
                    name: "".to_string(),
                    initializer: None,
                    type_type: "".to_string(),
                  },
                  fields: vec![
                    Field { name: "id".to_string(), initializer: None, type_type: "String".to_string() },
                    Field { name: "token".to_string(), initializer: None, type_type: "UUID".to_string() },
                    Field { name: "status".to_string(), initializer: Some("ReservationStatus.OPEN".to_string()), type_type: "ReservationStatus".to_string() },
                    Field { name: "expiresAt".to_string(), initializer: None, type_type: "LocalDateTime".to_string() },
                    Field { name: "createdAt".to_string(), initializer: None, type_type: "LocalDateTime".to_string() },
                    Field { name: "screeningId".to_string(), initializer: None, type_type: "String".to_string() },
                    Field { name: "screeningStartTime".to_string(), initializer: None, type_type: "LocalDateTime".to_string() },
                    Field { name: "name".to_string(), initializer: None, type_type: "String".to_string() },
                    Field { name: "surname".to_string(), initializer: None, type_type: "String".to_string() },
                    Field { name: "tickets".to_string(), initializer: None, type_type: "Set<Ticket>".to_string() },
                    Field { name: "totalPrice".to_string(), initializer: None, type_type: "BigDecimal".to_string() },
                  ],
                },
              ],
            }
          ],
        },
        BoundedContext {
          name: "User".to_string(),
          aggregates: vec![
            Aggregate {
              name: "User".to_string(),
              description: "".to_string(),
              entities: vec![
                Entity {
                  name: "User".to_string(),
                  description: "".to_string(),
                  is_aggregate_root: false,
                  identify: Field {
                    name: "".to_string(),
                    initializer: None,
                    type_type: "".to_string(),
                  },
                  fields: vec![
                    Field { name: "id".to_string(), initializer: None, type_type: "UUID".to_string() },
                    Field { name: "mobile".to_string(), initializer: None, type_type: "String".to_string() },
                    Field { name: "email".to_string(), initializer: None, type_type: "String".to_string() },
                    Field { name: "username".to_string(), initializer: None, type_type: "String".to_string() },
                    Field { name: "password".to_string(), initializer: None, type_type: "String".to_string() },
                    Field { name: "address".to_string(), initializer: None, type_type: "String".to_string() },
                  ],
                }
              ],
            }
          ],
        }],
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
        },
        ContextRelation {
          source: "Reservation".to_string(),
          target: "User".to_string(),
          connection_type: PositiveDirected,
          source_type: vec![],
          target_type: vec![],
        }],
      implementations: vec![],
      layered: None
    });
  }
}
