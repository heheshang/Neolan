# Design: User Profile Page Improvements

## Overview

This document describes the design decisions and architecture for improving the user profile and settings pages in NeoLan. The improvements focus on better user experience through inline editing, tabbed navigation, and progressive disclosure.

## Goals

1. **Reduce navigation friction** - Allow quick profile edits without leaving the profile page
2. **Improve discoverability** - Use tabs and search to make settings easier to find
3. **Reduce cognitive load** - Hide advanced options behind toggles
4. **Better mobile experience** - Responsive design optimized for touch interactions

## Architecture

### Component Hierarchy

```
App.vue
└── UserProfileHeader.vue (existing)
└── RouterView
    ├── UserProfileView.vue (enhanced)
    │   ├── ProfileDisplayCard.vue (read-only mode)
    │   ├── ProfileEditCard.vue (inline editing mode - NEW)
    │   ├── AvatarPicker.vue (NEW)
    │   └── QuickActions.vue (NEW)
    │
    └── SettingsView.vue (refactored)
        ├── SettingsHeader.vue (NEW - includes search)
        ├── TabNavigation.vue (NEW)
        ├── IdentitySettings.vue (extracted - NEW)
        ├── NetworkSettings.vue (extracted - NEW)
        ├── SecuritySettings.vue (extracted - NEW)
        ├── MessageSettings.vue (extracted - NEW)
        ├── FileSettings.vue (extracted - NEW)
        └── SystemSettings.vue (extracted - NEW)
```

### State Management

#### UserProfileView State

```typescript
interface UserProfileState {
  // Current display mode
  isEditing: boolean

  // Form data for editing
  editForm: {
    username: string
    hostname: string
    status: string
    avatar: string | null
  }

  // Original values for cancel/rollback
  originalValues: {
    username: string
    hostname: string
    status: string
    avatar: string | null
  }

  // UI state
  isSaving: boolean
  showAvatarPicker: boolean
  validationErrors: Record<string, string>
}
```

#### SettingsView State

```typescript
interface SettingsState {
  // Active tab
  currentTab: 'identity' | 'network' | 'security' | 'messages' | 'files' | 'system'

  // Tab-specific unsaved changes tracking
  unsavedChanges: Set<string>

  // Search state
  searchQuery: string
  searchResults: SearchResult[]
  currentMatchIndex: number

  // Advanced options visibility
  showAdvanced: boolean

  // Form data per tab
  tabData: Record<string, ConfigDto>
}
```

## Key Design Decisions

### 1. Inline Editing vs. Navigation

**Decision**: Support both inline editing (for common fields) and full settings page.

**Rationale**:
- Common task (changing username) should be quick - inline editing
- Complex configuration needs full settings page with tabs
- Power users can go directly to settings, casual users stay on profile

**Trade-offs**:
- Pro: Faster for common tasks
- Pro: Lower cognitive load for simple changes
- Con: More complex state management (two edit modes)
- Con: Potential confusion about where to edit what

### 2. Tabbed Navigation for Settings

**Decision**: Organize settings into 6 tabs with URL-based navigation.

**Rationale**:
- Groups related settings together
- Reduces scrolling and cognitive load
- Enables deep linking to specific settings
- Matches common patterns in other applications

**Tab Organization**:
1. **Identity** - User-facing info (username, hostname, avatar, status)
2. **Network** - Network configuration (ports, addresses)
3. **Security** - Encryption and security settings
4. **Messages** - Message handling preferences
5. **Files** - File transfer settings
6. **System** - Advanced system options

**Trade-offs**:
- Pro: Easier to find specific settings
- Pro: Can deep link to share settings URLs
- Con: More clicks to see all settings
- Con: Need to manage unsaved changes per tab

### 3. Progressive Disclosure for Advanced Options

**Decision**: Hide advanced options behind "Show Advanced" toggle.

**Rationale**:
- Reduces visual clutter and cognitive load
- Prevents accidental changes to critical settings
- Followes principle of "safe defaults"
- Advanced users can still access when needed

**Advanced Settings Criteria**:
- Settings that could break connectivity if misconfigured
- Technical debugging options (log levels, custom ports)
- Security-sensitive options (encryption keys)

**Trade-offs**:
- Pro: Cleaner interface for most users
- Pro: Safer defaults
- Con: Advanced settings less discoverable
- Con: Need to remember where advanced options are hidden

### 4. Avatar Customization

**Decision**: Support emoji picker first, image upload as optional enhancement.

**Rationale**:
- Emoji picker is simpler to implement (no backend storage changes)
- Emoji are lightweight and work well with cyberpunk aesthetic
- Image upload requires file storage strategy (filesystem vs. embedded)

**Implementation Phases**:
1. Phase 1: Emoji picker only
2. Phase 2: Add image upload (optional)

**Trade-offs**:
- Pro: Fast to implement with emojis
- Pro: No backend changes required for emoji storage
- Con: Limited customization with emojis only
- Con: Image upload requires additional infrastructure

### 5. Settings Search

**Decision**: Implement client-side search across all settings with tab auto-switching.

**Rationale**:
- Helps users quickly find specific settings
- No backend changes required
- Works offline
- Can highlight matches across tabs

**Search Scope**:
- Setting labels (e.g., "UDP PORT")
- Setting descriptions/hints
- Field values (optional)

**Trade-offs**:
- Pro: No backend complexity
- Pro: Fast response time
- Con: Limited to current settings structure
- Con: Need to keep search index in sync

## Data Flow

### Inline Editing Flow

```
User clicks "Edit Profile"
  ↓
UserProfileView sets isEditing = true
  ↓
ProfileDisplayCard replaced with ProfileEditCard
  ↓
User modifies fields (username, hostname, status)
  ↓
User clicks "Save" OR presses Ctrl+S
  ↓
Validate all fields
  ├─ Invalid → Show inline errors, stay in edit mode
  └─ Valid → Call configStore.saveConfig()
      ↓
    Success? → Yes: Update local state, exit edit mode, show success toast
            → No: Show error toast, stay in edit mode
```

### Settings Tab Navigation Flow

```
User navigates to /settings?tab=network
  ↓
SettingsView reads query param, sets currentTab = 'network'
  ↓
TabNavigation highlights 'network' tab
  ↓
NetworkSettings component is displayed
  ↓
User modifies network settings
  ↓
User clicks "Save"
  ↓
Validate network settings
  ├─ Invalid → Show errors, stay on network tab
  └─ Valid → Call configStore.saveConfig()
      ↓
    Success? → Yes: Clear unsaved changes, show success toast
            → No: Show error toast
  ↓
User switches to different tab
  ↓
Check unsavedChanges Set
  ├─ Has unsaved changes → Show confirmation dialog
  └─ No unsaved changes → Switch tab immediately
```

### Settings Search Flow

```
User types "encryption" in search box
  ↓
SettingsView calls searchSettings(query)
  ↓
Search across all tabs for matching settings
  ↓
Collect matches with tab and field info
  ↓
If match found in different tab:
  ↓
Auto-switch to that tab
  ↓
Highlight matching fields
  ↓
Display match count "3 matches found"
  ↓
User clicks "Next Match" or presses F3
  ↓
Move focus to next matching field
  ↓
Scroll field into view
```

## Implementation Considerations

### Performance

**Optimization Strategies**:
1. **Lazy load tab components** - Only render active tab content
2. **Debounce search input** - Wait 300ms after typing before searching
3. **Memoize validation results** - Don't re-validate unchanged fields
4. **Optimistic UI updates** - Update UI immediately, rollback on error

**Metrics to Track**:
- Profile page load time: target < 100ms
- Settings page load time: target < 150ms
- Tab switch time: target < 50ms
- Search response time: target < 100ms

### Accessibility

**Keyboard Navigation**:
- Tab: Navigate between form fields
- Arrow keys: Navigate tabs
- Ctrl+S / Cmd+S: Save changes
- Esc: Cancel editing
- Enter: Activate focused button
- Space: Toggle focused checkbox

**Screen Reader Support**:
- ARIA labels for all inputs
- ARIA roles for tab navigation
- Live regions for error messages
- Focus indicators always visible

**Color Contrast**:
- All text meets WCAG AA contrast ratio (4.5:1)
- Error states use color + icon indication
- Focus states use high-contrast outlines

### Mobile Responsiveness

**Touch Targets**:
- Minimum 44x44 pixels for all interactive elements
- Spacing between touch targets to prevent accidental taps

**Layout Adaptations**:
- Single column layout for profile cards
- Horizontally scrolling tabs
- Sticky action buttons at bottom of screen
- Full-width inputs on mobile

**Virtual Keyboard Handling**:
- Scroll active field into view when keyboard appears
- Keep save/cancel buttons visible
- Prevent body scroll when editing

### Error Handling

**Validation Strategy**:
1. **Real-time validation** - Validate on field blur
2. **Cross-field validation** - Revalidate related fields on change
3. **Save-time validation** - Final validation before save
4. **Backend validation** - Handle server-side validation errors

**Error Display**:
- Inline errors below each field
- Error summary on save attempt
- Toast notifications for backend errors
- Destructive actions require confirmation

### State Persistence

**What to Persist**:
- URL query params for tab state (?tab=network)
- Session storage for:
  - Advanced options toggle state
  - Search query (optional)
  - Draft unsaved changes (optional)

**What NOT to Persist**:
- Form field values (security risk)
- Passwords or sensitive data
- Temporary validation errors

## Migration Strategy

### Phase 1: Profile Page Inline Editing (Week 1)
- Create ProfileEditCard component
- Update UserProfileView with edit mode
- Add avatar picker
- Test and validate

### Phase 2: Settings Page Tabs (Week 2)
- Create TabNavigation component
- Extract settings into tab components
- Refactor SettingsView
- Test tab navigation and URL state

### Phase 3: Search and Advanced Features (Week 3)
- Add settings search
- Implement progressive disclosure
- Add settings import/export
- Enhanced validation

### Phase 4: Polish and Testing (Week 4)
- Visual design improvements
- Mobile responsiveness
- Accessibility audit
- Performance optimization
- User acceptance testing

## Future Enhancements

### Potential Future Features
1. **Settings profiles** - Save/load different configuration profiles
2. **Settings recommendations** - Suggest optimal settings based on usage
3. **Configuration diff viewer** - Show changes before applying imports
4. **Settings reset by category** - Reset only network settings, not all
5. **Collaborative settings** - Share configuration snippets with other users

### Extensibility Points
- Plugin system for adding custom settings tabs
- Custom validation rules per field
- Themeable settings page layout
- Internationalization support for settings labels

## Security Considerations

### Input Validation
- All user inputs must be validated client-side
- Server-side validation is authoritative
- Sanitize all inputs before display

### Sensitive Data Handling
- Encryption keys should be masked in UI
- Never log sensitive configuration values
- Clear sensitive data from memory after use

### Settings Import/Export
- Validate imported settings against schema
- Warn about potentially dangerous settings
- Require confirmation for imports
- Optionally exclude sensitive data from exports

## Conclusion

This design balances usability improvements with implementation complexity. The phased approach allows us to deliver value incrementally while managing risk. The component architecture supports future enhancements while maintaining code quality and testability.
