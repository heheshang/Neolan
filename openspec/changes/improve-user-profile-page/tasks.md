# Implementation Tasks

## Phase 1: Profile Page Inline Editing

### 1.1 Create ProfileEditCard component
- [ ] Create `src/components/ProfileEditCard.vue` with inline editing form
- [ ] Implement username input field with validation
- [ ] Implement hostname input field with validation
- [ ] Add status/description textarea field
- [ ] Add save/cancel action buttons
- [ ] Implement keyboard shortcuts (Ctrl+S to save, Esc to cancel)
- [ ] Add loading states during save

### 1.2 Update UserProfileView for editing mode
- [ ] Add `isEditing` state management
- [ ] Create toggle between display/edit modes
- [ ] Integrate ProfileEditCard component
- [ ] Update ProfileDisplayCard to show current values
- [ ] Add "Edit Profile" button to trigger edit mode
- [ ] Add "Advanced Settings" button linking to Settings page
- [ ] Implement optimistic UI updates with rollback on error
- [ ] Add success/error toast notifications

### 1.3 Add avatar customization
- [ ] Create `src/components/AvatarPicker.vue` component
- [ ] Implement emoji picker for avatar selection
- [ ] Add avatar upload functionality (optional image)
- [ ] Update backend ConfigDto to include avatar field
- [ ] Update backend config storage for avatar persistence
- [ ] Add avatar preview in profile header and display card

### 1.4 Test profile page editing
- [ ] Test inline editing flow end-to-end
- [ ] Test save success and error scenarios
- [ ] Test keyboard shortcuts (Ctrl+S, Esc)
- [ ] Test mobile responsiveness of edit mode
- [ ] Verify real-time updates to profile header
- [ ] Test concurrent edit prevention

## Phase 2: Settings Page Tabbed Navigation

### 2.1 Create TabNavigation component
- [ ] Create `src/components/TabNavigation.vue` component
- [ ] Implement tab rendering with active state
- [ ] Add tab click handlers
- [ ] Support URL query params for tab state (?tab=network)
- [ ] Add keyboard navigation (arrow keys, Home/End)
- [ ] Implement tab indicators and transitions

### 2.2 Extract settings sections into tab components
- [ ] Create `src/components/settings/IdentitySettings.vue`
- [ ] Create `src/components/settings/NetworkSettings.vue`
- [ ] Create `src/components/settings/SecuritySettings.vue`
- [ ] Create `src/components/settings/MessageSettings.vue`
- [ ] Create `src/components/settings/FileSettings.vue`
- [ ] Create `src/components/settings/SystemSettings.vue`
- [ ] Move relevant form fields from SettingsView to each tab component

### 2.3 Refactor SettingsView to use tabs
- [ ] Update SettingsView template to use TabNavigation component
- [ ] Implement tab state management with reactive currentTab
- [ ] Add URL query param sync for tab state
- [ ] Implement tab-specific save actions
- [ ] Add "unsaved changes" detection per tab
- [ ] Add tab switch confirmation with unsaved changes
- [ ] Maintain existing validation logic in each tab

### 2.4 Add settings search/filter
- [ ] Add search input field in settings header
- [ ] Implement search across all setting labels and descriptions
- [ ] Highlight matching settings in current tab
- [ ] Add "next/previous match" navigation
- [ ] Show match count indicator
- [ ] Clear search on tab change

## Phase 3: UX and Visual Improvements

### 3.1 Progressive disclosure for advanced options
- [ ] Identify advanced settings (encryption key, custom ports, etc.)
- [ ] Add "Show Advanced Options" toggle in each tab
- [ ] Hide advanced fields behind toggle
- [ ] Animate show/hide transitions
- [ ] Persist advanced toggle state in session storage

### 3.2 Enhance form validation
- [ ] Add real-time validation on input blur
- [ ] Display inline error messages below fields
- [ ] Add field-level success indicators
- [ ] Implement cross-field validation (e.g., port ranges)
- [ ] Add validation summary on save attempt
- [ ] Improve error message clarity

### 3.3 Improve visual design
- [ ] Enhance card layouts with better spacing
- [ ] Add color-coded section accents per tab
- [ ] Improve input field focus states
- [ ] Add subtle animations for interactions
- [ ] Enhance button hover and active states
- [ ] Improve contrast for accessibility

### 3.4 Mobile responsiveness improvements
- [ ] Test and optimize tab layout for mobile
- [ ] Implement horizontal tab scrolling for mobile
- [ ] Optimize form layouts for small screens
- [ ] Test touch interactions (swipe, tap targets)
- [ ] Improve mobile keyboard handling

## Phase 4: Additional Features

### 4.1 Settings import/export
- [ ] Add "Export Settings" button to SettingsView
- [ ] Implement settings JSON export
- [ ] Add "Import Settings" button with file picker
- [ ] Implement settings JSON import with validation
- [ ] Add import confirmation dialog
- [ ] Test import/export roundtrip

### 4.2 Reset improvements
- [ ] Add tab-specific reset to defaults
- [ ] Improve reset confirmation dialog
- [ ] Show preview of changes before reset
- [ ] Add undo option after reset

### 4.3 Add unit tests
- [ ] Add tests for ProfileEditCard component
- [ ] Add tests for TabNavigation component
- [ ] Add tests for inline editing state management
- [ ] Add tests for tab state management
- [ ] Add integration tests for settings save/load

### 4.4 Update documentation
- [ ] Update user-profile spec with new requirements
- [ ] Add inline documentation for new components
- [ ] Update CLAUDE.md with new component architecture
- [ ] Document keyboard shortcuts in UI

## Verification

### Manual Testing
- [ ] Test inline editing on profile page
- [ ] Test tab navigation in settings
- [ ] Test settings search functionality
- [ ] Test avatar customization
- [ ] Test on desktop (Chrome, Firefox, Edge)
- [ ] Test on mobile (responsive design)
- [ ] Test keyboard navigation throughout

### Regression Testing
- [ ] Verify all existing settings still work
- [ ] Verify config save/load still functions
- [ ] Verify validation rules unchanged
- [ ] Run existing unit tests
- [ ] Run existing integration tests

### Performance Testing
- [ ] Profile page load time < 100ms
- [ ] Settings page load time < 150ms
- [ ] Tab switch time < 50ms
- [ ] Search response time < 100ms
- [ ] No memory leaks in tab switching

### User Acceptance Testing
- [ ] Users can edit profile without navigating to settings
- [ ] Users can find settings quickly using tabs
- [ ] Users can search for specific settings
- [ ] Overall satisfaction rating ≥ 4/5
