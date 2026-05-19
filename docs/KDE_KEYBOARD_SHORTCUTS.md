# RustRover 实际快捷键速查表

> 基于你电脑上 **实际 Bundled Keymap** 文件提取的配置
> 当前键位方案：**Default for KDE** (继承自 Default for XWin → $default)
> 注意：KDE 方案为避免与系统快捷键冲突，调整了部分键位

## 通用操作

| 功能 | 快捷键 |
|------|--------|
| 查找动作 (Find Action) | `Ctrl+Shift+A` |
| 打开设置 (Settings) | `Ctrl+Alt+S` |
| 项目结构 (Project Structure) | `Ctrl+Alt+Shift+S` |
| 返回编辑器 (Editor Escape) | `Esc` |
| 聚焦编辑器 (Focus Editor) | `Esc` |

## 基本编辑

| 功能 | 快捷键 |
|------|--------|
| 撤销 (Undo) | `Ctrl+Z / Alt+Backspace` |
| 重做 (Redo) | `Ctrl+Shift+Z / Alt+Shift+Backspace` |
| 剪切 (Cut) | `Ctrl+X / Shift+Delete` |
| 复制 (Copy) | `Ctrl+C / Ctrl+Insert` |
| 粘贴 (Paste) | `Ctrl+V / Shift+Insert` |
| 全选 (Select All) | `Ctrl+A` |
| 删除 (Delete) | `Delete` |
| 纯文本粘贴 (Paste as Plain Text) | `Ctrl+Alt+Shift+V` |

## 代码编辑

| 功能 | 快捷键 |
|------|--------|
| 基础代码补全 (Basic Completion) | `Ctrl+Space` |
| 智能类型补全 (Smart Completion) | `Ctrl+Shift+Space` |
| 类名补全 (Class Name Completion) | `Ctrl+Alt+Space` |
| 显示上下文操作 / 快速修复 | `Alt+Enter` |
| 行注释 | `Ctrl+/ / Ctrl+/` |
| 块注释 | `Ctrl+Shift+/ / Ctrl+Shift+/` |
| 复制行 (Duplicate Lines) | `Ctrl+D` |
| 删除行 (Delete Line) | `Ctrl+Y` |
| 缩进行 (Indent) | `Tab` |
| 取消缩进 (Unindent) | `Shift+Tab` |
| 合并行 (Join Lines) | `Ctrl+Shift+J` |
| 拆分行 (Split Line) | `Ctrl+Enter` |
| 切换大小写 (Toggle Case) | `Ctrl+Shift+U` |
| 扩展选区 (Extend Selection) | `Ctrl+W` |
| 缩小选区 (Shrink Selection) | `Ctrl+Shift+W` |
| 格式化代码 (Reformat Code) | `Shift+Alt+L` |
| 自动缩进 (Auto-Indent) | `Ctrl+Alt+I` |
| 优化导入 (Optimize Imports) | `Ctrl+Alt+O` |
| 编译 (Compile) | `Ctrl+9` |
| 环绕 (Surround With) | `Ctrl+Alt+Shift+B / Ctrl+Alt+T` |

## 导航

| 功能 | 快捷键 |
|------|--------|
| 跳转到声明 (Go to Declaration) | `Ctrl+B` |
| 跳转到实现 (Go to Implementation) | `Ctrl+Alt+B` |
| 跳转到类型声明 (Go to Type Declaration) | `Ctrl+Shift+B` |
| 跳转到父类/方法 (Go to Super) | `Ctrl+U` |
| 跳转到测试 (Go to Test) | `Ctrl+Shift+T` |
| 跳转到相关项 (Go to Related) | `Ctrl+Alt+Home` |
| 跳转到类 (Go to Class) | `Ctrl+N` |
| 跳转到文件 (Go to File) | `Ctrl+Shift+N` |
| 跳转到符号 (Go to Symbol) | `Ctrl+Shift+Alt+N` |
| 跳转到行 (Go to Line) | `Ctrl+G` |
| 最近文件 (Recent Files) | `Ctrl+E` |
| 最近位置 (Recent Locations) | `Ctrl+Shift+E` |
| 最后编辑位置 (Last Edit Location) | `F12` |
| 后退 (Back) | `Ctrl+Alt+←` |
| 前进 (Forward) | `Ctrl+Alt+→` |
| 文件结构弹出 (File Structure) | `Ctrl+0` |
| 文件路径 (File Path) | `Ctrl+Alt+Shift+2` |
| 查找用法 (Find Usages) | `Shift+Alt+7` |
| 文件中查找用法 (Find Usages in File) | `Ctrl+7` |
| 高亮文件中用法 | `Shift+Ctrl+7` |
| 显示用法列表 (Show Usages) | `Ctrl+Alt+7` |
| 下一个错误 (Next Error) | `F2` |
| 上一个错误 (Previous Error) | `Shift+F2` |

## 查找替换

| 功能 | 快捷键 |
|------|--------|
| 当前文件查找 (Find) | `Ctrl+F / Shift+Alt+3` |
| 当前文件替换 (Replace) | `Ctrl+R` |
| 查找下一个 (Find Next) | `F3 / Ctrl+L` |
| 查找上一个 (Find Previous) | `Shift+F3 / Ctrl+Shift+L` |

## 代码查看

| 功能 | 快捷键 |
|------|--------|
| 快速文档 (Quick Documentation) | `Ctrl+Q` |
| 表达式类型信息 | `Ctrl+Shift+P` |
| 错误描述 (Error Description) | `Ctrl+1` |
| 类型层级 (Type Hierarchy) | `Ctrl+H` |
| 方法层级 (Method Hierarchy) | `Ctrl+Shift+H` |
| 调用层级 (Call Hierarchy) | `Ctrl+Alt+H` |
| 切换文档渲染 | `Ctrl+Alt+Q` |

## 重构

| 功能 | 快捷键 |
|------|--------|
| 重命名 (Rename) | `Shift+F6` |
| 更改签名 (Change Signature) | `Ctrl+6` |
| 移动 (Move) | `F6` |
| 安全删除 (Safe Delete) | `Alt+Delete` |
| 内联 (Inline) | `Ctrl+Alt+N` |
| 提取方法 (Extract Method) | `Ctrl+Alt+M` |
| 重构操作列表 (Refactor This) | `Ctrl+Alt+Shift+T` |

## 运行与调试

| 功能 | 快捷键 |
|------|--------|
| 运行 (Run) | `Shift+F10` |
| 调试 (Debug) | `Shift+F9` |
| 运行上下文配置 (Run Context) | `Ctrl+Shift+F10` |
| 选择运行配置 (Choose Run Config) | `Alt+Shift+F10` |
| 选择调试配置 (Choose Debug Config) | `Alt+Shift+F9` |
| 单步跳过 (Step Over) | `F8` |
| 单步进入 (Step Into) | `F7` |
| 单步跳出 (Step Out) | `Shift+F8` |
| 恢复程序 (Resume) | `F9` |
| 停止 (Stop) | `Ctrl+2` |
| 计算表达式 (Evaluate Expression) | `Shift+Alt+8` |
| 快速计算表达式 (Quick Evaluate) | `Ctrl+Alt+8` |
| 重新运行 (Rerun) | `Ctrl+5` |
| 切换行断点 (Toggle Breakpoint) | `Ctrl+8` |
| 查看所有断点 (View Breakpoints) | `Shift+Ctrl+8` |
| 切换覆盖率 (Toggle Coverage) | `Ctrl+Alt+6` |
| 显示执行点 (Show Execution Point) | `Shift+Alt+0` |

## 工具窗口

| 功能 | 快捷键 |
|------|--------|
| Project 项目文件窗口 | `Alt+1` |
| 查找结果窗口 (Find) | `Alt+3` |
| 运行窗口 (Run) | `Alt+4` |
| 调试窗口 (Debug) | `Alt+5` |
| 问题窗口 (Problems) | `Alt+6` |
| 结构窗口 (Structure) | `Alt+7` |
| 服务窗口 (Services) | `Alt+8` |
| 提交窗口 (Commit) | `Alt+0` |
| 版本控制窗口 (Version Control) | `Alt+9` |
| 关闭活动标签 (Close Active Tab) | `Shift+Ctrl+4` |
| 关闭当前文件 (Close) | `Ctrl+4` |

## 分割窗口

| 功能 | 快捷键 |
|------|--------|

## 书签

| 功能 | 快捷键 |
|------|--------|
| 切换书签 (Toggle Bookmark) | `F11` |
| 带助记符书签 | `Ctrl+F11` |
| 显示所有书签 (Show Bookmarks) | `Shift+F11` |

## 标签页

| 功能 | 快捷键 |
|------|--------|
| 下一个标签页 (Next Tab) | `Alt+→` |
| 上一个标签页 (Previous Tab) | `Alt+←` |

## 版本控制 (Git)

| 功能 | 快捷键 |
|------|--------|
| VCS 快速操作菜单 | `Alt+`` |

## 光标移动

| 功能 | 快捷键 |
|------|--------|
| 向上滚动一行 (Scroll Up) | `Ctrl+↑` |
| 向下滚动一行 (Scroll Down) | `Ctrl+↓` |
| 向上翻页 (Page Up) | `PgUp` |
| 向下翻页 (Page Down) | `PgDn` |
| 跳到上一个单词 (Previous Word) | `Ctrl+←` |
| 跳到下一个单词 (Next Word) | `Ctrl+→` |
| 跳到文件头 (File Start) | `Ctrl+Home` |
| 跳到文件尾 (File End) | `Ctrl+End` |
| 滚动到光标居中 (Scroll to Center) | `Ctrl+M` |

## 视图缩放

| 功能 | 快捷键 |
|------|--------|
| 增大字体 (Increase Font) | `Alt+Shift+.` |
| 减小字体 (Decrease Font) | `Alt+Shift+,` |
| 缩放放大 (Zoom In) | `Shift+Ctrl+Alt+=` |
| 缩放缩小 (Zoom Out) | `Shift+Ctrl+Alt+-` |
| 重置缩放 (Reset Zoom) | `Shift+Ctrl+Alt+0` |

## 其他

| 功能 | 快捷键 |
|------|--------|
| 刷新 (Refresh) | `Ctrl+5` |
| 强制刷新 (Force Refresh) | `Ctrl+Shift+5` |
| 恢复默认布局 | `Shift+Alt+F12` |
| 在...中选择 (Select In) | `Shift+Alt+1` |
| 停止 (Stop) | `Ctrl+2` |

---
> 生成方式：从 `platform-ide-impl.jar` 中的 $default.xml + Default for XWin.xml + Default for KDE.xml 合并导出
> 注意：部分插件动态绑定的快捷键（如 VCS Git 操作: `Ctrl+K` 提交, `Ctrl+Shift+K` 推送等）不在此 keymap 文件中，以 IDE 实际表现为准
> 如要查看所有快捷键，可在 IDE 内按 `Ctrl+Shift+A` 搜索 "Keyboard Shortcuts PDF" 导出完整参考卡
