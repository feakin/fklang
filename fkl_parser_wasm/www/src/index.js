import {FklParser} from "@feakin/fkl-wasm-web";

let template = `
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
`;
let parser = new FklParser(template);
console.log(parser.parse());
console.log(parser.to_dot());

document.writeln(parser.to_dot());
