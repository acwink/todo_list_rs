### 命令行ToDoList

#### 功能分析

添加待办事项：用户可以输入新的待办事项并将其添加到列表中。
列出所有待办事项：显示所有待办事项，包括完成状态。
删除待办事项：用户可以通过待办事项的索引来删除它。
标记待办事项为完成：用户可以选择一个待办事项并将其标记为已完成。
退出程序：允许用户退出应用程序。

需要使用 clap 进行命令行参数解析
我们使用txt来持久化我们的todo任务
no | desc | status
1  | Learning English | 0
2  | tEST | 1
3 | TEST | 2

STATUS:
0: 等待处理
1：处理中
2：处理完成

