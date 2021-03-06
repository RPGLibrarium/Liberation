---
title: Get Titles by RpgSystem
layout: page
nav_link: Get Titles by RpgSystem
nav_order: 313
nav_level: 3
lang: en
---

```
GET /v1/rpgsystems/{systemid}
```
### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=false %}

### Responses
**Content-Type:** application/json
- [200: OK](#200getTitlesByRpgSystem)
- [400: Bad Request](#400getTitlesByRpgSystem)
- [401: Unauthroized](#401getTitlesByRpgSystem)
- [403: Forbidden](#403getTitlesByRpgSystem)
- [404: Not Found](#404getTitlesByRpgSystem)
- [429: Too Many Requests](#429getTitlesByRpgSystem)

#### 200: Ok
```json
{
  "rpgsystem": {
    "id": 123829,
    "name": "Wege der Helden",
    "titles": [
      {
        "id": 123829,
        "name": "Wege der Helden",
        "language": "DE",
        "publisher": "Ulisses",
        "year": 2007,
        "coverimage": "https://example.com/wege-der-helden.jpg",
        "stock": 5,
        "available": 5,
      }
    ]
  }
}
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/forbidden.md %}

{% include_relative partials/notFound.md %}

{% include_relative partials/tooManyRequests.md %}
