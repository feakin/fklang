# FKL LSP for Monaco Editor

Sample: https://github.com/silvanshade/tower-lsp-web-demo

# Binding API

## Domain Event

Subscribe / Publish / Event / Flow

```kotlin
DomainEvent createBook {
  """bla bla"""
  // location with modules
  qualified: "$module:com.example.book"", 
  
  notication ?
  api ?
  
  test {
     host: ""
     token: ""
  }
  
  input {
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
    validate  { 
      required { min: 3, max: 10 }
      pattern { regex: "^[a-zA-Z0-9]+$" }
      range { min: 1, max: 100 }
    } 
  } 
  output {
     Struct {
        "id" : "number"
     }
     validate  { }
  } 
}
```

Container API ?

## Layered

```feakin

```

