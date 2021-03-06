---
title: Get RpgSystems
layout: page
nav_link: Get RpgSystems
nav_order: 311
nav_level: 3
lang: en
---

```
GET /v1/rpgsystems
```

### Parameters

| Name | Type  | Required | Description |
|:--------------|:--------|:----------:|:----------------------------------------------------------------------------------|
{% include_relative partials/param_authorization.md required=false %}

### Responses
**Content-Type:** application/json
- [200: OK](#200-ok)
- [400: Bad Request](#400-bad-request)
- [401: Unauthroized](#401-unauthorized)
- [429: Too Many Requests](#429-too-many-requests)

#### 200: Ok
```json
{
  "rpgsystems": [
    {
      "id": 123829,
      "name": "Das Schwarze Auge 4.1",
      "shortname": "DSA 4.1"
    }
  ]
}
```

{% include_relative partials/badRequest.md %}

{% include_relative partials/unauthorized.md %}

{% include_relative partials/tooManyRequests.md %}
