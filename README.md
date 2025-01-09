# Bayou Project & Package Manager

## Bayou Depot API

In order for a Bayou depot to be fully functional, it must correspond to the following HTTP API:

### Login

 - Endpoint: `/api/v1/me`
 - Method: `POST`

### Search

 - Endpoint: `/api/v1/packages`
 - Method: `GET`
 - Query Parameters:
    - `q`: The search query string.
    - `per_page`: Number of results, default 10, max 100.

### Info

 - Endpoint: `/api/v1/packages/{package_name}`
 - Method: `GET`

### Download

 - Endpoint: `/api/v1/packages/{package_name}/{version}`
 - Method: `GET`

### Publish

 - Endpoint: `/api/v1/packages/{package_name}`
 - Method: `PUT`
 - Authorization required

### Yank

 - Endpoint: `/api/v1/packages/{package_name}/{version}/yank`
 - Method: `DELETE`
 - Authorization required

### Unyank

 - Endpoint: `/api/v1/packages/{package_name}/{version}/unyank`
 - Method: `POST`
 - Authorization required

### Owners

#### List

 - Endpoint: `/api/v1/packages/{package_name}/owners`
 - Method: `GET`
 - Authorization required

#### Add

 - Endpoint: `/api/v1/packages/{package_name}/owners`
 - Method: `PUT`
 - Authorization required

#### Remove

 - Endpoint: `/api/v1/packages/{package_name}/owners`
 - Method: `DELETE`
 - Authorization required

## `bayou.ron` 