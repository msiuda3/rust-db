# rust-db
A simple key-value pair based database written in rust. The database uses a custom binary protocol for operations.

# About the protocol

## General Structure

Messages in this protocol consist of a series of fields. Each field contains specific information about the request or response.

### Common Fields

- **Version (1 byte)**: Specifies the protocol version being used.
- **Operation Type (1 byte)**: Defines the type of operation being performed (e.g., GET, PUT).

## GET Request

Used to retrieve the value associated with a key.

### Structure

| Field          | Size (bytes) | Description                        |
|----------------|--------------|------------------------------------|
| Version        | 1            | Protocol version                   |
| Operation Type | 1            | `0x01` for GET                     |
| Key Length     | 1            | Length of the key in bytes         |
| Key            | Variable     | Key (UTF-8 encoded string)         |

### Example

For a GET request to retrieve the key `"foo"` (version `0x01`):

| Field          | Value               |
|----------------|---------------------|
| Version        | `0x01`              |
| Operation Type | `0x01` (GET)        |
| Key Length     | `0x03`              |
| Key            | `foo` (UTF-8)       |

## GET Response

The response to a GET request.

### Structure

| Field          | Size (bytes) | Description                        |
|----------------|--------------|------------------------------------|
| Version        | 1            | Protocol version                   |
| Operation Type | 1            | `0x81` for GET response            |
| Status         | 1            | `0x00` if found, `0x01` if not found |
| Value Length   | 1            | Length of the value in bytes       |
| Value          | Variable     | Value (UTF-8 encoded string)       |

### Example

For a successful GET response with the value `"bar"` (version `0x01`):

| Field          | Value                |
|----------------|----------------------|
| Version        | `0x01`               |
| Operation Type | `0x81` (GET response)|
| Status         | `0x00` (found)       |
| Value Length   | `0x03`               |
| Value          | `bar` (UTF-8)        |

## PUT Request

Used to store a key-value pair in the database.

### Structure

| Field          | Size (bytes) | Description                        |
|----------------|--------------|------------------------------------|
| Version        | 1            | Protocol version                   |
| Operation Type | 1            | `0x02` for PUT                     |
| Key Length     | 1            | Length of the key in bytes         |
| Value Length   | 1            | Length of the value in bytes       |
| Key            | Variable     | Key (UTF-8 encoded string)         |
| Value          | Variable     | Value (UTF-8 encoded string)       |

### Example

For a PUT request to store the key `"foo"` with the value `"bar"` (version `0x01`):

| Field          | Value                |
|----------------|----------------------|
| Version        | `0x01`               |
| Operation Type | `0x02` (PUT)         |
| Key Length     | `0x03`               |
| Value Length   | `0x03`               |
| Key            | `foo` (UTF-8)        |
| Value          | `bar` (UTF-8)        |

## PUT Response

The response to a PUT request.

### Structure

| Field          | Size (bytes) | Description                        |
|----------------|--------------|------------------------------------|
| Version        | 1            | Protocol version                   |
| Operation Type | 1            | `0x82` for PUT response            |
| Status         | 1            | `0x00` if success, `0x01` if failure |

### Example

For a successful PUT response (version `0x01`):

| Field          | Value                |
|----------------|----------------------|
| Version        | `0x01`               |
| Operation Type | `0x82` (PUT response)|
| Status         | `0x00` (success)     |


### Notes

- The protocol uses **Big-Endian** encoding for all multi-byte fields.
- All keys and values are UTF-8 encoded strings.
- Future versions may introduce new operation types or extend existing message structures with additional fields (e.g., checksums).

