# Change: Improve User Profile Settings Page

## Why

The current user profile and settings pages have several usability issues:

1. **Profile page is read-only**: Users must navigate to settings page to make any changes, creating an inefficient workflow
2. **Settings page is overwhelming**: All configuration sections are displayed at once, making it difficult to find specific settings
3. **Poor visual hierarchy**: Important user identity settings are buried among technical network configuration
4. **No quick access**: Cannot quickly update username or nickname without navigating through multiple sections
5. **Tabbed navigation missing**: Users have to scroll through long page to find specific configuration areas

User feedback indicates:
- "I just want to change my username, why do I need to scroll through all these network settings?"
- "The settings page is too long, I can't find what I'm looking for"
- "Would be nice if I could edit my profile right on the profile page"

## What Changes

### Profile Page Enhancements
- **Inline editing mode**: Allow users to edit username and hostname directly on the profile page
- **Quick action buttons**: Add "Edit Profile" and "Advanced Settings" buttons for different editing contexts
- **Avatar customization**: Add ability to set custom avatar (emoji or image)
- **Status message**: Add editable status/description field

### Settings Page Improvements
- **Tabbed navigation**: Organize settings into tabs (Identity, Network, Security, Messages, Files, System)
- **Improved grouping**: Better visual separation between related settings
- **Quick search**: Add search/filter functionality to quickly locate settings
- **Progressive disclosure**: Hide advanced options behind "Advanced" toggle to reduce cognitive load

### UX Improvements
- **Better navigation flow**: Profile page → Quick Edit (inline) OR Advanced Settings (tabbed)
- **Save indicators**: Clear visual feedback when settings are modified
- **Keyboard shortcuts**: Support Ctrl+S to save, Esc to cancel
- **Mobile responsiveness**: Better mobile layout for profile and settings pages

### Visual Enhancements
- **Improved card layouts**: Better spacing and visual hierarchy
- **Color-coded sections**: Use accent colors to differentiate setting categories
- **Enhanced form controls**: Better input field styling and focus states
- **Micro-interactions**: Subtle animations for better feedback

## Impact

- **Affected specs**:
  - Modify `user-profile` spec to add inline editing requirements
  - Add new requirements for tabbed navigation and progressive disclosure
- **Affected code**:
  - Frontend: `src/views/UserProfileView.vue` - Add inline editing mode
  - Frontend: `src/views/SettingsView.vue` - Implement tabbed navigation
  - Frontend: `src/components/` - Create new ProfileEdit component, TabNavigation component
  - Frontend: `src/stores/config.ts` - May need updates for better state management
- **Risk**: Medium - UI changes may require testing across different screen sizes
- **Breaking changes**: None - all changes are additive improvements

## Technical Details

### Proposed Component Structure

```
src/views/
├── UserProfileView.vue (enhanced)
│   ├── ProfileDisplayCard.vue (new - read-only view)
│   ├── ProfileEditCard.vue (new - inline editing)
│   └── QuickActions.vue (new - edit/settings buttons)
│
├── SettingsView.vue (refactored)
│   ├── TabNavigation.vue (new - tabs component)
│   └── SettingsTabs/
│       ├── IdentitySettings.vue (extracted)
│       ├── NetworkSettings.vue (extracted)
│       ├── SecuritySettings.vue (extracted)
│       ├── MessageSettings.vue (extracted)
│       ├── FileSettings.vue (extracted)
│       └── SystemSettings.vue (extracted)
```

### Tab Navigation Design

**Tabs:**
1. **Identity** (first tab, focused)
   - Username, hostname
   - Avatar, nickname
   - Status message
   - Display preferences

2. **Network**
   - Bind address, UDP port
   - TCP port range
   - Heartbeat interval, peer timeout

3. **Security**
   - Encryption settings
   - Encryption key

4. **Messages**
   - Offline message retention
   - Message formatting options

5. **Files**
   - Auto-accept files
   - File save directory

6. **System**
   - Log level
   - Advanced options

### Inline Editing State Management

```typescript
// UserProfileView.vue state
const isEditing = ref(false)
const editMode = ref<'quick' | 'advanced'>('quick')

// Quick edit: username, hostname, status inline
// Advanced edit: navigate to Settings page with Identity tab pre-selected
```

### Implementation Phases

**Phase 1: Profile Page Inline Editing**
- Add inline edit button to profile page
- Create ProfileEditCard component
- Implement username/hostname/status inline editing
- Add save/cancel buttons with keyboard shortcuts

**Phase 2: Settings Page Tabbed Navigation**
- Implement TabNavigation component
- Extract settings sections into separate tab components
- Add tab state management (URL query params for deep linking)
- Implement tab-specific save/reset actions

**Phase 3: UX and Visual Polish**
- Add search/filter functionality
- Implement progressive disclosure for advanced options
- Enhance mobile responsiveness
- Add micro-interactions and animations

**Phase 4: Additional Features**
- Avatar customization (emoji picker or image upload)
- Enhanced form validation with inline error messages
- Settings import/export
- Reset to defaults confirmation improvements

## Dependencies

- Requires Vue 3 Composition API (already in use)
- May need vue-router for tab URL state management (already available)
- No additional backend changes required (uses existing config APIs)

## Success Criteria

1. Users can edit username/hostname/status directly on profile page without navigation
2. Settings page uses tabbed navigation with clear section labels
3. Reduced time to complete common configuration tasks (measured via user testing)
4. No regression in existing functionality
5. Mobile layout remains functional and improved
6. All existing tests pass
