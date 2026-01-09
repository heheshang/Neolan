## ADDED Requirements

### Requirement: User Profile Header Display
The application SHALL display the current user's profile information in a prominent header bar at the top of the main interface.

#### Scenario: Display user profile in header
- **WHEN** the application loads
- **THEN** the user's username and hostname SHALL be displayed in the header bar
- **AND** the information SHALL be retrieved from the application configuration

#### Scenario: Real-time update on config change
- **WHEN** the user changes their username or hostname in settings
- **THEN** the header display SHALL update to reflect the new values
- **AND** the update SHALL occur without requiring application restart

### Requirement: User Profile Detail Page
The application SHALL provide a dedicated user profile page showing complete user configuration information.

#### Scenario: Navigate to user profile page
- **WHEN** the user clicks on the profile header
- **THEN** the application SHALL navigate to the user profile detail page
- **AND** the page SHALL display complete user information including username, hostname, and network settings

#### Scenario: Display read-only user information
- **WHEN** viewing the user profile page
- **THEN** the page SHALL display current user configuration in a read-only format
- **AND** a button SHALL be provided to navigate to the settings page for modifications

### Requirement: User Profile Data Source
User profile information SHALL be retrieved from the existing application configuration system.

#### Scenario: Load profile from config
- **WHEN** the user profile components load
- **THEN** they SHALL fetch configuration data using the existing `getConfig()` API
- **AND** they SHALL subscribe to configuration changes for real-time updates

#### Scenario: Handle missing or default configuration
- **WHEN** configuration is not yet loaded
- **THEN** the components SHALL display default values or loading state
- **AND** the application SHALL fall back to system-provided defaults from `whoami` crate
