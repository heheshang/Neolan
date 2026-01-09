# Change: Add User Profile Display

## Why

用户希望在前端界面中清晰地看到当前用户的名称和信息。当前系统已经在后端通过 `whoami` crate 获取了用户名和主机名，并且可以在设置页面中修改，但在主界面中缺少明显的用户身份展示。参考 IPMsg 消息格式中的用户信息显示（如 `陈俞辛`），需要在前端添加用户信息的展示。

## What Changes

- **在顶部导航栏添加用户信息显示**：显示当前用户的用户名和主机名
- **创建用户个人信息组件/页面**：展示完整的用户配置信息
- **确保配置变更时实时更新显示**
- **支持从配置中读取和显示用户信息**

## Impact

- Affected specs: New spec `user-profile` for user profile display capability
- Affected code:
  - Frontend: 新增 `UserProfileHeader.vue` 组件
  - Frontend: 新增 `UserProfileView.vue` 页面
  - Frontend: 修改路由配置
  - Frontend: 修改 `App.vue` 或主布局以包含顶部用户信息栏
- Risk: Low - 主要是 UI 变更，使用现有的配置 API
