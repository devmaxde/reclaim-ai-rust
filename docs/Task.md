# Task

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | Unique identifier of the task | [optional]
**created** | Option<**String**> | Creation timestamp | [optional]
**updated** | Option<**String**> | Last update timestamp | [optional]
**title** | **String** | Task title | 
**notes** | Option<**String**> | Task notes | [optional]
**event_category** | Option<[**models::EventCategory**](EventCategory.md)> |  | [optional]
**event_sub_type** | Option<**String**> | Event subtype | [optional]
**time_scheme_id** | Option<**String**> | Time scheme ID (custom hours) | [optional]
**time_chunks_required** | Option<**i32**> | Time chunks required | [optional]
**min_chunk_size** | Option<**i32**> | Minimum chunk size | [optional]
**max_chunk_size** | Option<**i32**> | Maximum chunk size | [optional]
**priority** | Option<[**models::TaskPriority**](TaskPriority.md)> |  | [optional]
**on_deck** | Option<**bool**> | Task is on deck | [optional]
**always_private** | Option<**bool**> | Task is always private | [optional]
**status** | Option<[**models::TaskStatus**](TaskStatus.md)> |  | [optional]
**due** | Option<**String**> | Due date | [optional]
**snooze_until** | Option<**String**> | Snooze until date | [optional]
**index** | Option<**f64**> | Task index | [optional]
**event_color** | Option<[**models::EventColor**](EventColor.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


