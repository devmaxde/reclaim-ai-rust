# \DefaultApi

All URIs are relative to *https://api.app.reclaim.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**api_planner_add_time_task_id_post**](DefaultApi.md#api_planner_add_time_task_id_post) | **POST** /api/planner/add-time/task/{id} | Add time to a task
[**api_planner_clear_exceptions_task_id_post**](DefaultApi.md#api_planner_clear_exceptions_task_id_post) | **POST** /api/planner/clear-exceptions/task/{id} | Clear exceptions for a task
[**api_planner_done_task_id_post**](DefaultApi.md#api_planner_done_task_id_post) | **POST** /api/planner/done/task/{id} | Mark a task as complete
[**api_planner_log_work_task_id_post**](DefaultApi.md#api_planner_log_work_task_id_post) | **POST** /api/planner/log-work/task/{id} | Log work on a task
[**api_planner_prioritize_task_id_post**](DefaultApi.md#api_planner_prioritize_task_id_post) | **POST** /api/planner/prioritize/task/{id} | Prioritize a task
[**api_planner_start_task_id_post**](DefaultApi.md#api_planner_start_task_id_post) | **POST** /api/planner/start/task/{id} | Start a task
[**api_planner_stop_task_id_post**](DefaultApi.md#api_planner_stop_task_id_post) | **POST** /api/planner/stop/task/{id} | Stop a task
[**api_planner_unarchive_task_id_post**](DefaultApi.md#api_planner_unarchive_task_id_post) | **POST** /api/planner/unarchive/task/{id} | Mark a task as incomplete
[**api_tasks_get**](DefaultApi.md#api_tasks_get) | **GET** /api/tasks | List all tasks
[**api_tasks_id_delete**](DefaultApi.md#api_tasks_id_delete) | **DELETE** /api/tasks/{id} | Delete a task
[**api_tasks_id_get**](DefaultApi.md#api_tasks_id_get) | **GET** /api/tasks/{id} | Get a task by ID
[**api_tasks_id_patch**](DefaultApi.md#api_tasks_id_patch) | **PATCH** /api/tasks/{id} | Update a task
[**api_tasks_post**](DefaultApi.md#api_tasks_post) | **POST** /api/tasks | Create a new task
[**api_tasks_reindex_by_due_patch**](DefaultApi.md#api_tasks_reindex_by_due_patch) | **PATCH** /api/tasks/reindex-by-due | Reindex tasks by due date
[**api_timeschemes_get**](DefaultApi.md#api_timeschemes_get) | **GET** /api/timeschemes | List all time schemes



## api_planner_add_time_task_id_post

> models::ApiPlannerDoneTaskIdPost200Response api_planner_add_time_task_id_post(id, minutes)
Add time to a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |
**minutes** | **i32** | Minutes to add (rounded to nearest 15) | [required] |

### Return type

[**models::ApiPlannerDoneTaskIdPost200Response**](_api_planner_done_task__id__post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_planner_clear_exceptions_task_id_post

> models::ApiPlannerDoneTaskIdPost200Response api_planner_clear_exceptions_task_id_post(id)
Clear exceptions for a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |

### Return type

[**models::ApiPlannerDoneTaskIdPost200Response**](_api_planner_done_task__id__post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_planner_done_task_id_post

> models::ApiPlannerDoneTaskIdPost200Response api_planner_done_task_id_post(id)
Mark a task as complete

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |

### Return type

[**models::ApiPlannerDoneTaskIdPost200Response**](_api_planner_done_task__id__post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_planner_log_work_task_id_post

> models::ApiPlannerDoneTaskIdPost200Response api_planner_log_work_task_id_post(id, minutes, end)
Log work on a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |
**minutes** | **i32** | Minutes worked | [required] |
**end** | Option<**String**> | End time of work |  |

### Return type

[**models::ApiPlannerDoneTaskIdPost200Response**](_api_planner_done_task__id__post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_planner_prioritize_task_id_post

> models::ApiPlannerDoneTaskIdPost200Response api_planner_prioritize_task_id_post(id)
Prioritize a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |

### Return type

[**models::ApiPlannerDoneTaskIdPost200Response**](_api_planner_done_task__id__post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_planner_start_task_id_post

> models::ApiPlannerDoneTaskIdPost200Response api_planner_start_task_id_post(id)
Start a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |

### Return type

[**models::ApiPlannerDoneTaskIdPost200Response**](_api_planner_done_task__id__post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_planner_stop_task_id_post

> models::ApiPlannerDoneTaskIdPost200Response api_planner_stop_task_id_post(id)
Stop a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |

### Return type

[**models::ApiPlannerDoneTaskIdPost200Response**](_api_planner_done_task__id__post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_planner_unarchive_task_id_post

> models::ApiPlannerDoneTaskIdPost200Response api_planner_unarchive_task_id_post(id)
Mark a task as incomplete

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |

### Return type

[**models::ApiPlannerDoneTaskIdPost200Response**](_api_planner_done_task__id__post_200_response.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tasks_get

> Vec<models::Task> api_tasks_get()
List all tasks

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Task>**](Task.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tasks_id_delete

> api_tasks_id_delete(id)
Delete a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tasks_id_get

> models::Task api_tasks_id_get(id)
Get a task by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tasks_id_patch

> models::Task api_tasks_id_patch(id, task)
Update a task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** | Task ID | [required] |
**task** | [**Task**](Task.md) |  | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tasks_post

> models::Task api_tasks_post(task)
Create a new task

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**task** | [**Task**](Task.md) |  | [required] |

### Return type

[**models::Task**](Task.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_tasks_reindex_by_due_patch

> api_tasks_reindex_by_due_patch()
Reindex tasks by due date

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## api_timeschemes_get

> Vec<models::Hours> api_timeschemes_get()
List all time schemes

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::Hours>**](Hours.md)

### Authorization

[BearerAuth](../README.md#BearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

