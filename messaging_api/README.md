# Rust API client for line-messaging-api

This document describes LINE Messaging API.


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: 0.0.1
- Package version: 0.0.1
- Generator version: 7.12.0
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `line-messaging-api` and add the following to `Cargo.toml` under `[dependencies]`:

```
line-messaging-api = { path = "./line-messaging-api" }
```

## Documentation for API Endpoints

All URIs are relative to *https://api.line.me*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*MessagingApiApi* | [**broadcast**](docs/MessagingApiApi.md#broadcast) | **Post** /v2/bot/message/broadcast | 
*MessagingApiApi* | [**cancel_default_rich_menu**](docs/MessagingApiApi.md#cancel_default_rich_menu) | **Delete** /v2/bot/user/all/richmenu | 
*MessagingApiApi* | [**create_rich_menu**](docs/MessagingApiApi.md#create_rich_menu) | **Post** /v2/bot/richmenu | 
*MessagingApiApi* | [**create_rich_menu_alias**](docs/MessagingApiApi.md#create_rich_menu_alias) | **Post** /v2/bot/richmenu/alias | 
*MessagingApiApi* | [**delete_rich_menu**](docs/MessagingApiApi.md#delete_rich_menu) | **Delete** /v2/bot/richmenu/{richMenuId} | 
*MessagingApiApi* | [**delete_rich_menu_alias**](docs/MessagingApiApi.md#delete_rich_menu_alias) | **Delete** /v2/bot/richmenu/alias/{richMenuAliasId} | 
*MessagingApiApi* | [**get_aggregation_unit_name_list**](docs/MessagingApiApi.md#get_aggregation_unit_name_list) | **Get** /v2/bot/message/aggregation/list | 
*MessagingApiApi* | [**get_aggregation_unit_usage**](docs/MessagingApiApi.md#get_aggregation_unit_usage) | **Get** /v2/bot/message/aggregation/info | 
*MessagingApiApi* | [**get_bot_info**](docs/MessagingApiApi.md#get_bot_info) | **Get** /v2/bot/info | 
*MessagingApiApi* | [**get_default_rich_menu_id**](docs/MessagingApiApi.md#get_default_rich_menu_id) | **Get** /v2/bot/user/all/richmenu | 
*MessagingApiApi* | [**get_followers**](docs/MessagingApiApi.md#get_followers) | **Get** /v2/bot/followers/ids | 
*MessagingApiApi* | [**get_group_member_count**](docs/MessagingApiApi.md#get_group_member_count) | **Get** /v2/bot/group/{groupId}/members/count | 
*MessagingApiApi* | [**get_group_member_profile**](docs/MessagingApiApi.md#get_group_member_profile) | **Get** /v2/bot/group/{groupId}/member/{userId} | 
*MessagingApiApi* | [**get_group_members_ids**](docs/MessagingApiApi.md#get_group_members_ids) | **Get** /v2/bot/group/{groupId}/members/ids | 
*MessagingApiApi* | [**get_group_summary**](docs/MessagingApiApi.md#get_group_summary) | **Get** /v2/bot/group/{groupId}/summary | 
*MessagingApiApi* | [**get_joined_membership_users**](docs/MessagingApiApi.md#get_joined_membership_users) | **Get** /v2/bot/membership/{membershipId}/users/ids | 
*MessagingApiApi* | [**get_membership_list**](docs/MessagingApiApi.md#get_membership_list) | **Get** /v2/bot/membership/list | 
*MessagingApiApi* | [**get_membership_subscription**](docs/MessagingApiApi.md#get_membership_subscription) | **Get** /v2/bot/membership/subscription/{userId} | 
*MessagingApiApi* | [**get_message_quota**](docs/MessagingApiApi.md#get_message_quota) | **Get** /v2/bot/message/quota | 
*MessagingApiApi* | [**get_message_quota_consumption**](docs/MessagingApiApi.md#get_message_quota_consumption) | **Get** /v2/bot/message/quota/consumption | 
*MessagingApiApi* | [**get_narrowcast_progress**](docs/MessagingApiApi.md#get_narrowcast_progress) | **Get** /v2/bot/message/progress/narrowcast | 
*MessagingApiApi* | [**get_number_of_sent_broadcast_messages**](docs/MessagingApiApi.md#get_number_of_sent_broadcast_messages) | **Get** /v2/bot/message/delivery/broadcast | 
*MessagingApiApi* | [**get_number_of_sent_multicast_messages**](docs/MessagingApiApi.md#get_number_of_sent_multicast_messages) | **Get** /v2/bot/message/delivery/multicast | 
*MessagingApiApi* | [**get_number_of_sent_push_messages**](docs/MessagingApiApi.md#get_number_of_sent_push_messages) | **Get** /v2/bot/message/delivery/push | 
*MessagingApiApi* | [**get_number_of_sent_reply_messages**](docs/MessagingApiApi.md#get_number_of_sent_reply_messages) | **Get** /v2/bot/message/delivery/reply | 
*MessagingApiApi* | [**get_pnp_message_statistics**](docs/MessagingApiApi.md#get_pnp_message_statistics) | **Get** /v2/bot/message/delivery/pnp | 
*MessagingApiApi* | [**get_profile**](docs/MessagingApiApi.md#get_profile) | **Get** /v2/bot/profile/{userId} | 
*MessagingApiApi* | [**get_rich_menu**](docs/MessagingApiApi.md#get_rich_menu) | **Get** /v2/bot/richmenu/{richMenuId} | 
*MessagingApiApi* | [**get_rich_menu_alias**](docs/MessagingApiApi.md#get_rich_menu_alias) | **Get** /v2/bot/richmenu/alias/{richMenuAliasId} | 
*MessagingApiApi* | [**get_rich_menu_alias_list**](docs/MessagingApiApi.md#get_rich_menu_alias_list) | **Get** /v2/bot/richmenu/alias/list | 
*MessagingApiApi* | [**get_rich_menu_batch_progress**](docs/MessagingApiApi.md#get_rich_menu_batch_progress) | **Get** /v2/bot/richmenu/progress/batch | 
*MessagingApiApi* | [**get_rich_menu_id_of_user**](docs/MessagingApiApi.md#get_rich_menu_id_of_user) | **Get** /v2/bot/user/{userId}/richmenu | 
*MessagingApiApi* | [**get_rich_menu_list**](docs/MessagingApiApi.md#get_rich_menu_list) | **Get** /v2/bot/richmenu/list | 
*MessagingApiApi* | [**get_room_member_count**](docs/MessagingApiApi.md#get_room_member_count) | **Get** /v2/bot/room/{roomId}/members/count | 
*MessagingApiApi* | [**get_room_member_profile**](docs/MessagingApiApi.md#get_room_member_profile) | **Get** /v2/bot/room/{roomId}/member/{userId} | 
*MessagingApiApi* | [**get_room_members_ids**](docs/MessagingApiApi.md#get_room_members_ids) | **Get** /v2/bot/room/{roomId}/members/ids | 
*MessagingApiApi* | [**get_webhook_endpoint**](docs/MessagingApiApi.md#get_webhook_endpoint) | **Get** /v2/bot/channel/webhook/endpoint | 
*MessagingApiApi* | [**issue_link_token**](docs/MessagingApiApi.md#issue_link_token) | **Post** /v2/bot/user/{userId}/linkToken | 
*MessagingApiApi* | [**leave_group**](docs/MessagingApiApi.md#leave_group) | **Post** /v2/bot/group/{groupId}/leave | 
*MessagingApiApi* | [**leave_room**](docs/MessagingApiApi.md#leave_room) | **Post** /v2/bot/room/{roomId}/leave | 
*MessagingApiApi* | [**link_rich_menu_id_to_user**](docs/MessagingApiApi.md#link_rich_menu_id_to_user) | **Post** /v2/bot/user/{userId}/richmenu/{richMenuId} | 
*MessagingApiApi* | [**link_rich_menu_id_to_users**](docs/MessagingApiApi.md#link_rich_menu_id_to_users) | **Post** /v2/bot/richmenu/bulk/link | 
*MessagingApiApi* | [**mark_messages_as_read**](docs/MessagingApiApi.md#mark_messages_as_read) | **Post** /v2/bot/message/markAsRead | 
*MessagingApiApi* | [**multicast**](docs/MessagingApiApi.md#multicast) | **Post** /v2/bot/message/multicast | 
*MessagingApiApi* | [**narrowcast**](docs/MessagingApiApi.md#narrowcast) | **Post** /v2/bot/message/narrowcast | 
*MessagingApiApi* | [**push_message**](docs/MessagingApiApi.md#push_message) | **Post** /v2/bot/message/push | 
*MessagingApiApi* | [**push_messages_by_phone**](docs/MessagingApiApi.md#push_messages_by_phone) | **Post** /bot/pnp/push | 
*MessagingApiApi* | [**reply_message**](docs/MessagingApiApi.md#reply_message) | **Post** /v2/bot/message/reply | 
*MessagingApiApi* | [**rich_menu_batch**](docs/MessagingApiApi.md#rich_menu_batch) | **Post** /v2/bot/richmenu/batch | 
*MessagingApiApi* | [**set_default_rich_menu**](docs/MessagingApiApi.md#set_default_rich_menu) | **Post** /v2/bot/user/all/richmenu/{richMenuId} | 
*MessagingApiApi* | [**set_webhook_endpoint**](docs/MessagingApiApi.md#set_webhook_endpoint) | **Put** /v2/bot/channel/webhook/endpoint | 
*MessagingApiApi* | [**show_loading_animation**](docs/MessagingApiApi.md#show_loading_animation) | **Post** /v2/bot/chat/loading/start | 
*MessagingApiApi* | [**test_webhook_endpoint**](docs/MessagingApiApi.md#test_webhook_endpoint) | **Post** /v2/bot/channel/webhook/test | 
*MessagingApiApi* | [**unlink_rich_menu_id_from_user**](docs/MessagingApiApi.md#unlink_rich_menu_id_from_user) | **Delete** /v2/bot/user/{userId}/richmenu | 
*MessagingApiApi* | [**unlink_rich_menu_id_from_users**](docs/MessagingApiApi.md#unlink_rich_menu_id_from_users) | **Post** /v2/bot/richmenu/bulk/unlink | 
*MessagingApiApi* | [**update_rich_menu_alias**](docs/MessagingApiApi.md#update_rich_menu_alias) | **Post** /v2/bot/richmenu/alias/{richMenuAliasId} | 
*MessagingApiApi* | [**validate_broadcast**](docs/MessagingApiApi.md#validate_broadcast) | **Post** /v2/bot/message/validate/broadcast | 
*MessagingApiApi* | [**validate_multicast**](docs/MessagingApiApi.md#validate_multicast) | **Post** /v2/bot/message/validate/multicast | 
*MessagingApiApi* | [**validate_narrowcast**](docs/MessagingApiApi.md#validate_narrowcast) | **Post** /v2/bot/message/validate/narrowcast | 
*MessagingApiApi* | [**validate_push**](docs/MessagingApiApi.md#validate_push) | **Post** /v2/bot/message/validate/push | 
*MessagingApiApi* | [**validate_reply**](docs/MessagingApiApi.md#validate_reply) | **Post** /v2/bot/message/validate/reply | 
*MessagingApiApi* | [**validate_rich_menu_batch_request**](docs/MessagingApiApi.md#validate_rich_menu_batch_request) | **Post** /v2/bot/richmenu/validate/batch | 
*MessagingApiApi* | [**validate_rich_menu_object**](docs/MessagingApiApi.md#validate_rich_menu_object) | **Post** /v2/bot/richmenu/validate | 
*MessagingApiBlobApi* | [**get_message_content**](docs/MessagingApiBlobApi.md#get_message_content) | **Get** /v2/bot/message/{messageId}/content | 
*MessagingApiBlobApi* | [**get_message_content_preview**](docs/MessagingApiBlobApi.md#get_message_content_preview) | **Get** /v2/bot/message/{messageId}/content/preview | 
*MessagingApiBlobApi* | [**get_message_content_transcoding_by_message_id**](docs/MessagingApiBlobApi.md#get_message_content_transcoding_by_message_id) | **Get** /v2/bot/message/{messageId}/content/transcoding | 
*MessagingApiBlobApi* | [**get_rich_menu_image**](docs/MessagingApiBlobApi.md#get_rich_menu_image) | **Get** /v2/bot/richmenu/{richMenuId}/content | 
*MessagingApiBlobApi* | [**set_rich_menu_image**](docs/MessagingApiBlobApi.md#set_rich_menu_image) | **Post** /v2/bot/richmenu/{richMenuId}/content | 


## Documentation For Models

 - [Action](docs/Action.md)
 - [AgeDemographic](docs/AgeDemographic.md)
 - [AgeDemographicFilter](docs/AgeDemographicFilter.md)
 - [AllMentionTarget](docs/AllMentionTarget.md)
 - [AltUri](docs/AltUri.md)
 - [AppTypeDemographic](docs/AppTypeDemographic.md)
 - [AppTypeDemographicFilter](docs/AppTypeDemographicFilter.md)
 - [AreaDemographic](docs/AreaDemographic.md)
 - [AreaDemographicFilter](docs/AreaDemographicFilter.md)
 - [AudienceRecipient](docs/AudienceRecipient.md)
 - [AudioMessage](docs/AudioMessage.md)
 - [BotInfoResponse](docs/BotInfoResponse.md)
 - [BroadcastRequest](docs/BroadcastRequest.md)
 - [ButtonsTemplate](docs/ButtonsTemplate.md)
 - [CameraAction](docs/CameraAction.md)
 - [CameraRollAction](docs/CameraRollAction.md)
 - [CarouselColumn](docs/CarouselColumn.md)
 - [CarouselTemplate](docs/CarouselTemplate.md)
 - [ChatReference](docs/ChatReference.md)
 - [ClipboardAction](docs/ClipboardAction.md)
 - [ClipboardImagemapAction](docs/ClipboardImagemapAction.md)
 - [ConfirmTemplate](docs/ConfirmTemplate.md)
 - [CreateRichMenuAliasRequest](docs/CreateRichMenuAliasRequest.md)
 - [DatetimePickerAction](docs/DatetimePickerAction.md)
 - [DemographicFilter](docs/DemographicFilter.md)
 - [Emoji](docs/Emoji.md)
 - [EmojiSubstitutionObject](docs/EmojiSubstitutionObject.md)
 - [ErrorDetail](docs/ErrorDetail.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [Filter](docs/Filter.md)
 - [FlexBlockStyle](docs/FlexBlockStyle.md)
 - [FlexBox](docs/FlexBox.md)
 - [FlexBoxBackground](docs/FlexBoxBackground.md)
 - [FlexBoxBorderWidth](docs/FlexBoxBorderWidth.md)
 - [FlexBoxCornerRadius](docs/FlexBoxCornerRadius.md)
 - [FlexBoxLinearGradient](docs/FlexBoxLinearGradient.md)
 - [FlexBoxPadding](docs/FlexBoxPadding.md)
 - [FlexBoxSpacing](docs/FlexBoxSpacing.md)
 - [FlexBubble](docs/FlexBubble.md)
 - [FlexBubbleStyles](docs/FlexBubbleStyles.md)
 - [FlexButton](docs/FlexButton.md)
 - [FlexCarousel](docs/FlexCarousel.md)
 - [FlexComponent](docs/FlexComponent.md)
 - [FlexContainer](docs/FlexContainer.md)
 - [FlexFiller](docs/FlexFiller.md)
 - [FlexIcon](docs/FlexIcon.md)
 - [FlexIconSize](docs/FlexIconSize.md)
 - [FlexImage](docs/FlexImage.md)
 - [FlexImageSize](docs/FlexImageSize.md)
 - [FlexMargin](docs/FlexMargin.md)
 - [FlexMessage](docs/FlexMessage.md)
 - [FlexOffset](docs/FlexOffset.md)
 - [FlexSeparator](docs/FlexSeparator.md)
 - [FlexSpan](docs/FlexSpan.md)
 - [FlexSpanSize](docs/FlexSpanSize.md)
 - [FlexText](docs/FlexText.md)
 - [FlexTextFontSize](docs/FlexTextFontSize.md)
 - [FlexVideo](docs/FlexVideo.md)
 - [GenderDemographic](docs/GenderDemographic.md)
 - [GenderDemographicFilter](docs/GenderDemographicFilter.md)
 - [GetAggregationUnitNameListResponse](docs/GetAggregationUnitNameListResponse.md)
 - [GetAggregationUnitUsageResponse](docs/GetAggregationUnitUsageResponse.md)
 - [GetFollowersResponse](docs/GetFollowersResponse.md)
 - [GetJoinedMembershipUsersResponse](docs/GetJoinedMembershipUsersResponse.md)
 - [GetMembershipSubscriptionResponse](docs/GetMembershipSubscriptionResponse.md)
 - [GetMessageContentTranscodingResponse](docs/GetMessageContentTranscodingResponse.md)
 - [GetWebhookEndpointResponse](docs/GetWebhookEndpointResponse.md)
 - [GroupMemberCountResponse](docs/GroupMemberCountResponse.md)
 - [GroupSummaryResponse](docs/GroupSummaryResponse.md)
 - [GroupUserProfileResponse](docs/GroupUserProfileResponse.md)
 - [ImageCarouselColumn](docs/ImageCarouselColumn.md)
 - [ImageCarouselTemplate](docs/ImageCarouselTemplate.md)
 - [ImageMessage](docs/ImageMessage.md)
 - [ImagemapAction](docs/ImagemapAction.md)
 - [ImagemapArea](docs/ImagemapArea.md)
 - [ImagemapBaseSize](docs/ImagemapBaseSize.md)
 - [ImagemapExternalLink](docs/ImagemapExternalLink.md)
 - [ImagemapMessage](docs/ImagemapMessage.md)
 - [ImagemapVideo](docs/ImagemapVideo.md)
 - [IssueLinkTokenResponse](docs/IssueLinkTokenResponse.md)
 - [Limit](docs/Limit.md)
 - [LocationAction](docs/LocationAction.md)
 - [LocationMessage](docs/LocationMessage.md)
 - [MarkMessagesAsReadRequest](docs/MarkMessagesAsReadRequest.md)
 - [MembersIdsResponse](docs/MembersIdsResponse.md)
 - [Membership](docs/Membership.md)
 - [MembershipListResponse](docs/MembershipListResponse.md)
 - [MentionSubstitutionObject](docs/MentionSubstitutionObject.md)
 - [MentionTarget](docs/MentionTarget.md)
 - [Message](docs/Message.md)
 - [MessageAction](docs/MessageAction.md)
 - [MessageImagemapAction](docs/MessageImagemapAction.md)
 - [MessageQuotaResponse](docs/MessageQuotaResponse.md)
 - [MulticastRequest](docs/MulticastRequest.md)
 - [NarrowcastProgressResponse](docs/NarrowcastProgressResponse.md)
 - [NarrowcastRequest](docs/NarrowcastRequest.md)
 - [NumberOfMessagesResponse](docs/NumberOfMessagesResponse.md)
 - [OperatorDemographicFilter](docs/OperatorDemographicFilter.md)
 - [OperatorRecipient](docs/OperatorRecipient.md)
 - [PnpMessagesRequest](docs/PnpMessagesRequest.md)
 - [PostbackAction](docs/PostbackAction.md)
 - [PushMessageRequest](docs/PushMessageRequest.md)
 - [PushMessageResponse](docs/PushMessageResponse.md)
 - [QuickReply](docs/QuickReply.md)
 - [QuickReplyItem](docs/QuickReplyItem.md)
 - [QuotaConsumptionResponse](docs/QuotaConsumptionResponse.md)
 - [QuotaType](docs/QuotaType.md)
 - [Recipient](docs/Recipient.md)
 - [RedeliveryRecipient](docs/RedeliveryRecipient.md)
 - [ReplyMessageRequest](docs/ReplyMessageRequest.md)
 - [ReplyMessageResponse](docs/ReplyMessageResponse.md)
 - [RichMenuAliasListResponse](docs/RichMenuAliasListResponse.md)
 - [RichMenuAliasResponse](docs/RichMenuAliasResponse.md)
 - [RichMenuArea](docs/RichMenuArea.md)
 - [RichMenuBatchLinkOperation](docs/RichMenuBatchLinkOperation.md)
 - [RichMenuBatchOperation](docs/RichMenuBatchOperation.md)
 - [RichMenuBatchProgressPhase](docs/RichMenuBatchProgressPhase.md)
 - [RichMenuBatchProgressResponse](docs/RichMenuBatchProgressResponse.md)
 - [RichMenuBatchRequest](docs/RichMenuBatchRequest.md)
 - [RichMenuBatchUnlinkAllOperation](docs/RichMenuBatchUnlinkAllOperation.md)
 - [RichMenuBatchUnlinkOperation](docs/RichMenuBatchUnlinkOperation.md)
 - [RichMenuBounds](docs/RichMenuBounds.md)
 - [RichMenuBulkLinkRequest](docs/RichMenuBulkLinkRequest.md)
 - [RichMenuBulkUnlinkRequest](docs/RichMenuBulkUnlinkRequest.md)
 - [RichMenuIdResponse](docs/RichMenuIdResponse.md)
 - [RichMenuListResponse](docs/RichMenuListResponse.md)
 - [RichMenuRequest](docs/RichMenuRequest.md)
 - [RichMenuResponse](docs/RichMenuResponse.md)
 - [RichMenuSize](docs/RichMenuSize.md)
 - [RichMenuSwitchAction](docs/RichMenuSwitchAction.md)
 - [RoomMemberCountResponse](docs/RoomMemberCountResponse.md)
 - [RoomUserProfileResponse](docs/RoomUserProfileResponse.md)
 - [Sender](docs/Sender.md)
 - [SentMessage](docs/SentMessage.md)
 - [SetWebhookEndpointRequest](docs/SetWebhookEndpointRequest.md)
 - [ShowLoadingAnimationRequest](docs/ShowLoadingAnimationRequest.md)
 - [StickerMessage](docs/StickerMessage.md)
 - [SubscribedMembershipPlan](docs/SubscribedMembershipPlan.md)
 - [SubscribedMembershipUser](docs/SubscribedMembershipUser.md)
 - [Subscription](docs/Subscription.md)
 - [SubscriptionPeriodDemographic](docs/SubscriptionPeriodDemographic.md)
 - [SubscriptionPeriodDemographicFilter](docs/SubscriptionPeriodDemographicFilter.md)
 - [SubstitutionObject](docs/SubstitutionObject.md)
 - [Template](docs/Template.md)
 - [TemplateImageAspectRatio](docs/TemplateImageAspectRatio.md)
 - [TemplateImageSize](docs/TemplateImageSize.md)
 - [TemplateMessage](docs/TemplateMessage.md)
 - [TestWebhookEndpointRequest](docs/TestWebhookEndpointRequest.md)
 - [TestWebhookEndpointResponse](docs/TestWebhookEndpointResponse.md)
 - [TextMessage](docs/TextMessage.md)
 - [TextMessageV2](docs/TextMessageV2.md)
 - [UpdateRichMenuAliasRequest](docs/UpdateRichMenuAliasRequest.md)
 - [UriAction](docs/UriAction.md)
 - [UriImagemapAction](docs/UriImagemapAction.md)
 - [UserMentionTarget](docs/UserMentionTarget.md)
 - [UserProfileResponse](docs/UserProfileResponse.md)
 - [ValidateMessageRequest](docs/ValidateMessageRequest.md)
 - [VideoMessage](docs/VideoMessage.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author



