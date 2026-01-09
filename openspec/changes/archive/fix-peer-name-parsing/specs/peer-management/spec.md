## MODIFIED Requirements

### Requirement: FeiQ Message Username Extraction
The system SHALL correctly extract usernames from FeiQ BR_ENTRY messages where the username is contained in the content field rather than the sender_name field.

#### Scenario: Parse FeiQ BR_ENTRY message with Chinese username
- **GIVEN** a FeiQ format BR_ENTRY message: `1767153479:t0250254:DESKTOP-IOHG15K:6291459:陈俞辛`
- **WHEN** the message is parsed
- **THEN** the sender_name SHALL be `陈俞辛`
- **AND** the sender_host SHALL be `DESKTOP-IOHG15K`
- **AND** the msg_type SHALL be `6291459` (IPMSG_BR_ENTRY)

#### Scenario: Maintain backward compatibility with standard IPMsg format
- **GIVEN** a standard IPMsg message: `1:123:Alice:alice-pc:32:Hello`
- **WHEN** the message is parsed
- **THEN** the sender_name SHALL be `Alice`
- **AND** the sender_host SHALL be `alice-pc`
- **AND** backward compatibility SHALL be preserved

### Requirement: Peer Display Name Priority
The peer display name SHALL follow the priority: nickname > username > hostname > IP address.

#### Scenario: Display name from parsed username
- **WHEN** a FeiQ message with username `陈俞辛` is received
- **THEN** the peer's username field SHALL be set to `陈俞辛`
- **AND** the peer's display_name SHALL be `陈俞辛` (since username takes precedence)

#### Scenario: Fallback to hostname when username is empty
- **WHEN** a message has no username but has hostname
- **THEN** the display_name SHALL fallback to the hostname

### Requirement: BR_ENTRY Message Content Field Handling
For FeiQ BR_ENTRY messages (msg_type 6291459), the content field contains the username and MUST be used as the sender_name.

#### Scenario: Detect and handle BR_ENTRY message type
- **WHEN** parsing a FeiQ format message with msg_type 6291459
- **THEN** the system SHALL recognize this as a BR_ENTRY message
- **AND** the content field SHALL be extracted as the username
- **AND** the ProtocolMessage sender_name SHALL be set to the content value

#### Scenario: Non-BR_ENTRY FeiQ messages
- **WHEN** parsing a FeiQ format message that is not BR_ENTRY
- **THEN** the content field SHALL be treated as regular message content
- **AND** the sender_name SHALL follow standard IPMsg format from the sender_name field position
