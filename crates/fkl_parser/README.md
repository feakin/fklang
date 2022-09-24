# Feakin Knowledge Language

- [ ] DDD Model
- [ ] DSL Parser
  - [ ] DSL Syntax
  - [ ] Ast Model
- [ ] Code Binding
- [ ] Code Generator Model
- [ ] Workflow DSL
  - [ ] Event Storming
    - [ ] Role
    - [ ] Command
    - [ ] Event

# System Level

# Org. Modeling

- Application Level 
- Team Topology Model

# Continuous feedback

- C4

# Domain DSL Language

## DDD Building Blocks

- ContextMap
- Bounded Context
  - Shared Kernel
  - Anti-corruption Layer
- SubDomain
  - Core-domain
  - Supporting-domain
  - Generic-domain
- Layered Architecture
  - Domain
  - Application
  - Infrastructure
  - Interface

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
| attr_item          | :      | ID '=' ID [ (';' &#124; ',') ]                                               |
| aggregate_decl     | :      | [ 'Aggregate' ]  [ ID ] '{' entity_list '}'                                  |
|                    | &#124; | att_list                                                                     |
| entity_decl        | :      | [ 'Entity' ] [ ID ] '{' value_object_list '}'                                |
|                    | &#124; | att_list                                                                     |
| value_object__decl | :      | [ 'ValueObject' ] [ ID ] '{' value_list '}'                                  |
|                    | &#124; | att_list                                                                     |

## Draft

```kotlin
ContextMap {
  // with? type("Landscape")
  ShoppingCarContext(acl = NormalACL) <-> MallContext(acl = NormalACL);
}

Context ShoppingCarContext  {
  with? acl {
    with type("Anti-corruption Layer")
    with name("ShoppingCarACL")
    with description("Anti-corruption Layer for ShoppingCar")
  }
  with display("Shopping Car")
}

// render wtih UML styled?
SubDomain Cart {
  Aggregate Cart(display = "") {
    type = aggregateRoot, display = "Cart";
    something = "likethat";

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
}

// make owner ship?
Aggregate (owner="") {
  """ inline doc sample
just for test
"""

}

// global detail for Cart.
Entity Cart {

}

DomainLanguage Shopping {

}
```

API Binding

// [https://contextmapper.org/docs/mdsl/](https://contextmapper.org/docs/mdsl/)

```kotlin
ShoppingCarContext.API {
  // align to constructure for copy;
  model Address(id: Integeter);
  model AddressId(id: String);

  DomainEvent createAddress {
    Description "Creates new address for customer"
    PreValidate {
      // validate the input
    }
    Payload Address
    Response AddressId
  }
}

```

## Context Binding

binding source code to Context Map

```
ContextBinding {
  basePackage = se.citerus.dddsample.domain.model
}
```

## Styles

Styles:

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

```groovy
ContextMap {
  context1 {
    // AI Generator ?
    boundedContext1 {
      sharedKernel
      antiCorruptionLayer
      domain {
        subDomain
        coreDomain
        genericDomain
      }
      infrastructure
      application
      presentation
    }
  }
}
```

基于微服务的 DDD

核心域：Bounded Context, Unique Language,

支持域：Domain Object

通用域：Layered Architecture, Multiple Modules,


