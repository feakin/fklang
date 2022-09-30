# Feakin Knowledge Language

Fkl provide a two-way binding between design-implementation.

Syntax:

| decl               |        | usage                                                                        |
|--------------------|--------|------------------------------------------------------------------------------|
| context_map_decl   | :      | [ 'ContextMap' ] [ ID ] '{' (context_node_decl &#124; context_node_rel ) '}' |
|                    | &#124; | att_list                                                                     |
| context_node_decl  | :      | ['context'] [ID]                                                             |
| context_node_rel   | :      | [ ID ] rel_symbol [ ID ]                                                     |
| rel_symbol         | :      | ('->' &#124; '<-' &#124; '<->')                                              |                     
| context_decl       | :      | [ 'Context' ] [ ID ] '{' aggregate_list? '}'                                 |
|                    | &#124; | att_list                                                                     |
| att_list           | :      | attr_item+                                                                   |
| attr_item          | :      | ([ ID ] '=' [ value ] ','?)* ';'?                                            |
|                    | &#124; | ([ ID ] ':' [ value ] ','?)* ';'?                                            |
|                    | &#124; | [ ID ] ([ value, ',' ])*     ';'?                                            |
| aggregate_decl     | :      | [ 'Aggregate' ]  [ ID ] '{' entity_list '}'                                  |
|                    | &#124; | att_list                                                                     |
| entity_decl        | :      | [ 'Entity' ] [ ID ] '{' value_object_list '}'                                |
|                    | &#124; | att_list                                                                     |
| value_object__decl | :      | [ 'ValueObject' ] [ ID ] '{' value_list '}'                                  |
|                    | &#124; | att_list                                                                     |

Binding Syntax:

| decl                   |        | usage                                   |
|------------------------|--------|-----------------------------------------|
| source_set_decl        | :      | simple_source_set_decl                  |
|                        | &#124; | space_source_set_decl                   |
| space_source_set_decl  | :      | [ 'SourceSet' ] [ ID ] '{' att_list '}' |
| simple_source_set_decl | :      | [ 'SourceSet' ] [ ID ] '(' att_list ')' |
| implementation_decl    | :      | [ 'impl' ] [ID] '{' (inline_doc) '}'    |

Description Syntax:

| decl        |     | usage                                |
|-------------|-----|--------------------------------------|
| if_expr     | :   | [ 'if' ] '(' [ expression ]  ')'     |
| choose_expr | :   | [ 'choose' ] '(' [ expression ]  ')' |

## Draft

```feakin
ContextMap Ticket {

}

Context ShoppingCarContext  {

}

// render wtih UML styled?
Aggregate Cart {
  """ inline doc sample
just-demo for test
"""
  display = "Cart";
  DomainEvent CartCreated, CartItemAdded, CartItemRemoved, CartItemQuantityChanged, CartCheckedOut;
  DomainEvent CartItemQuantityChanged;

  // Concept or UML like ?
  // can be inside or outside of the Aggregate
  Entity Cart {
    // it's to many, can change in different way.
    ValueObject CartId
    ValueObject CartStatus
    ValueObject CartItem
    ValueObject CartItemQuantity
    ValueObject CartItemPrice
    ValueObject CartItemTotal
    ValueObject CartTotal 
  }
}

// global detail for Cart.
Entity Cart {

}

DomainLanguage Shopping {

}
```

## Typedef

### BuildIn Types

| Name        | Description                     |
|-------------|---------------------------------|
| identifier  | unique identifier               |
| binary      | Any binary data                 |
| bits        | A set of bits or flags          |
| boolean     | "true" or "false"               |
| enumeration | Enumerated strings              |
| string      | string                          |
| number      | Any number, can be float or int |
| optional ?  | Optional type ?                 |

### Container

```groovy
typedef(container) ContextMap {
 
}
```

| decl         |     | usage                                                 |
|--------------|-----|-------------------------------------------------------|
| typedef_decl | :   | [ 'typedef'] '(' metaType ')' ID '{' (decl_list) '}'; |
| decl_list    | :   | decl_item*                                            |
| decl_item    | :   | [ID] ':' decl_name                                    |

## Context Binding

binding source code to Context Map

```
Context Ticket {

}

binding Ticket {
  language: "Kotlin",
  layered: DDDLayered,
  qualified: "${moduleName}:se.citerus.dddsample.domain.model",
  // equals
  moduleName: "domain"
  package: "se.citerus.dddsample.domain.model"
}
```

## DomainEvent Implementation

Subscribe / Publish / Event / Flow

```kotlin
impl CinemaCreated {
  """bla bla"""
  // or binding to ?
  // binding: aggregate().

  // location with modules
  // default to "root" or ":"
  target: "${DomainObject}:com.example.book"
  // ?
  qualified: "${moduleName}:com.example.book", 
  
  endpoint {
    // message map
    notication ?
    // RPC API
    // HTTP API ?
    host: "http://localhost:8080"
    url: "/api/v1/books"
    
    method: "POST"  
      
    test {
       host: ""
       token: ""
    }
  }

  entity: Book
  input CreateBookRequest {
    struct {
      "title" : "string",
      "author" : "string",
      "price" : "number"
    }
    example {
      "title" : "The Lord of the Rings",
      "author" : "J.R.R. Tolkien",
      "price" : 29.99
    }
    validate {
      // title.length > 10 ? 
      title  {
        required { min: 3, max: 10 }
        pattern { regex: "^[a-zA-Z0-9]+$" }
        range { min: 1, max: 100 }
      }
    } 
  } 
  
  middle {
    via User get/update/delete/post userId 
    via Kafka send "book.created"
    // send "book.created" to Kafka
  }

  output CreateBookResponse {
     struct {
        "id" : "number"
     }
     validate  { }
  } 
  
  // contract-based development
  output CreateBookResponse(xpath="");
  input CreateBookResponse(sourceSet="PetSwagger" location="");
}
```

## SourceSet with Eco

plugins with extensions.

UML for Struct

```
Struct(sourceSet=DddUml, location="")

SourceSet DddUml {
  type: "puml",
  file: "ddd.puml"
}

// or
SourceSet(type="puml", file="ddd.puml")
```

```
SourceSet PetSwagger {
  file: "openapi.yaml",
  type: "openapi",
  prefix: "Pet"  // add prefix to items
}
```

Container API ?

## Layered Decl

```feakin
layered {
  dependency {
    "domain" -> "application"
    "application" -> "infrastructure"
    "interface" -> "infrastructure"
  }
  layer interface {
    package: "com.example.book"
  }
  layer domain {
    package: "com.example.domain"   
  }
  layer application {
    package: "com.example.application"
  }
  layer infrastructure {
    package: "com.example.infrastructure"
  }
}
```

## Description Decl

```feakin
description FakeCode {
  if (and ?) then {} else { }
  choose() {
    condition:
    condition:
  }
  done
  operator: <, >, >=, <=, ==, +, -, *, %, /, ? 
  // call
  via Entity send/receive Event;
}
```

## Style Decl

[Todo] ?

```kotlin
styles {

    // node
    element "Software System" {
        background #1168bd
        color #ffffff
    }
    element "Person" {
        shape person
        background #08427b
        color #ffffff
    }
    
    // edge
    relationship <tag> {
        thickness <integer>
        color #777777
        colour #777777
        dashed <true|false>
        style <solid|dashed|dotted>
        routing <Direct|Orthogonal|Curved>
        fontSize <integer>
        width <integer>
        position <integer: 0-100>
        opacity <integer: 0-100>
    }

}
```
