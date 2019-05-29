// =================================================================
//
//                           * WARNING *
//
//                    This file is generated!
//
//  Changes made to this file will be overwritten. If changes are
//  required to the generated code, the service_crategen project
//  must be updated to generate the changes.
//
// =================================================================

use std::error::Error;
use std::fmt;

#[allow(warnings)]
use futures::future;
use futures::Future;
use rusoto_core::credential::ProvideAwsCredentials;
use rusoto_core::region;
use rusoto_core::request::{BufferedHttpResponse, DispatchSignedRequest};
use rusoto_core::{Client, RusotoError, RusotoFuture};

use rusoto_core::proto;
use rusoto_core::signature::SignedRequest;
use serde_json;
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AddAttributesToFindingsRequest {
    /// <p>The array of attributes that you want to assign to specified findings.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
    /// <p>The ARNs that specify the findings that you want to assign attributes to.</p>
    #[serde(rename = "findingArns")]
    pub finding_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AddAttributesToFindingsResponse {
    /// <p>Attribute details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

/// <p>Used in the exception error that is thrown if you start an assessment run for an assessment target that includes an EC2 instance that is already participating in another started assessment run.</p>
#[derive(Default, Debug, Clone, PartialEq)]
pub struct AgentAlreadyRunningAssessment {
    /// <p>ID of the agent that is running on an EC2 instance that is already participating in another started assessment run.</p>
    pub agent_id: String,
    /// <p>The ARN of the assessment run that has already been started.</p>
    pub assessment_run_arn: String,
}

/// <p>Contains information about an Amazon Inspector agent. This data type is used as a request parameter in the <a>ListAssessmentRunAgents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AgentFilter {
    /// <p>The detailed health state of the agent. Values can be set to <b>IDLE</b>, <b>RUNNING</b>, <b>SHUTDOWN</b>, <b>UNHEALTHY</b>, <b>THROTTLED</b>, and <b>UNKNOWN</b>. </p>
    #[serde(rename = "agentHealthCodes")]
    pub agent_health_codes: Vec<String>,
    /// <p>The current health state of the agent. Values can be set to <b>HEALTHY</b> or <b>UNHEALTHY</b>.</p>
    #[serde(rename = "agentHealths")]
    pub agent_healths: Vec<String>,
}

/// <p>Used as a response element in the <a>PreviewAgents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AgentPreview {
    /// <p>The health status of the Amazon Inspector Agent.</p>
    #[serde(rename = "agentHealth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health: Option<String>,
    /// <p>The ID of the EC2 instance where the agent is installed.</p>
    #[serde(rename = "agentId")]
    pub agent_id: String,
    /// <p>The version of the Amazon Inspector Agent.</p>
    #[serde(rename = "agentVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_version: Option<String>,
    /// <p>The Auto Scaling group for the EC2 instance where the agent is installed.</p>
    #[serde(rename = "autoScalingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group: Option<String>,
    /// <p>The hostname of the EC2 instance on which the Amazon Inspector Agent is installed.</p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>The IP address of the EC2 instance on which the Amazon Inspector Agent is installed.</p>
    #[serde(rename = "ipv4Address")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_4_address: Option<String>,
    /// <p>The kernel version of the operating system running on the EC2 instance on which the Amazon Inspector Agent is installed.</p>
    #[serde(rename = "kernelVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kernel_version: Option<String>,
    /// <p>The operating system running on the EC2 instance on which the Amazon Inspector Agent is installed.</p>
    #[serde(rename = "operatingSystem")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_system: Option<String>,
}

/// <p>A snapshot of an Amazon Inspector assessment run that contains the findings of the assessment run .</p> <p>Used as the response element in the <a>DescribeAssessmentRuns</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssessmentRun {
    /// <p>The ARN of the assessment run.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The ARN of the assessment template that is associated with the assessment run.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
    /// <p>The assessment run completion time that corresponds to the rules packages evaluation completion time or failure.</p>
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    /// <p>The time when <a>StartAssessmentRun</a> was called.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>A Boolean value (true or false) that specifies whether the process of collecting data from the agents is completed.</p>
    #[serde(rename = "dataCollected")]
    pub data_collected: bool,
    /// <p>The duration of the assessment run.</p>
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i64,
    /// <p>Provides a total count of generated findings per severity.</p>
    #[serde(rename = "findingCounts")]
    pub finding_counts: ::std::collections::HashMap<String, i64>,
    /// <p>The auto-generated name for the assessment run.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>A list of notifications for the event subscriptions. A notification about a particular generated finding is added to this list only once.</p>
    #[serde(rename = "notifications")]
    pub notifications: Vec<AssessmentRunNotification>,
    /// <p>The rules packages selected for the assessment run.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
    /// <p>The time when <a>StartAssessmentRun</a> was called.</p>
    #[serde(rename = "startedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<f64>,
    /// <p>The state of the assessment run.</p>
    #[serde(rename = "state")]
    pub state: String,
    /// <p>The last time when the assessment run's state changed.</p>
    #[serde(rename = "stateChangedAt")]
    pub state_changed_at: f64,
    /// <p>A list of the assessment run state changes.</p>
    #[serde(rename = "stateChanges")]
    pub state_changes: Vec<AssessmentRunStateChange>,
    /// <p>The user-defined attributes that are assigned to every generated finding.</p>
    #[serde(rename = "userAttributesForFindings")]
    pub user_attributes_for_findings: Vec<Attribute>,
}

/// <p>Contains information about an Amazon Inspector agent. This data type is used as a response element in the <a>ListAssessmentRunAgents</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssessmentRunAgent {
    /// <p>The current health state of the agent.</p>
    #[serde(rename = "agentHealth")]
    pub agent_health: String,
    /// <p>The detailed health state of the agent.</p>
    #[serde(rename = "agentHealthCode")]
    pub agent_health_code: String,
    /// <p>The description for the agent health code.</p>
    #[serde(rename = "agentHealthDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_health_details: Option<String>,
    /// <p>The AWS account of the EC2 instance where the agent is installed.</p>
    #[serde(rename = "agentId")]
    pub agent_id: String,
    /// <p>The ARN of the assessment run that is associated with the agent.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>The Auto Scaling group of the EC2 instance that is specified by the agent ID.</p>
    #[serde(rename = "autoScalingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group: Option<String>,
    /// <p>The Amazon Inspector application data metrics that are collected by the agent.</p>
    #[serde(rename = "telemetryMetadata")]
    pub telemetry_metadata: Vec<TelemetryMetadata>,
}

/// <p>Used as the request parameter in the <a>ListAssessmentRuns</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssessmentRunFilter {
    /// <p>For a record to match a filter, the value that is specified for this data type property must inclusively match any value between the specified minimum and maximum values of the <b>completedAt</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "completionTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_time_range: Option<TimestampRange>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must inclusively match any value between the specified minimum and maximum values of the <b>durationInSeconds</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "durationRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_range: Option<DurationRange>,
    /// <p>For a record to match a filter, an explicit value or a string containing a wildcard that is specified for this data type property must match the value of the <b>assessmentRunName</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "namePattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_pattern: Option<String>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must be contained in the list of values of the <b>rulesPackages</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "rulesPackageArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_package_arns: Option<Vec<String>>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must inclusively match any value between the specified minimum and maximum values of the <b>startTime</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "startTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_range: Option<TimestampRange>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must match the <b>stateChangedAt</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "stateChangeTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_change_time_range: Option<TimestampRange>,
    /// <p>For a record to match a filter, one of the values specified for this data type property must be the exact match of the value of the <b>assessmentRunState</b> property of the <a>AssessmentRun</a> data type.</p>
    #[serde(rename = "states")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub states: Option<Vec<String>>,
}

/// <p>Used as one of the elements of the <a>AssessmentRun</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssessmentRunNotification {
    /// <p>The date of the notification.</p>
    #[serde(rename = "date")]
    pub date: f64,
    /// <p>The Boolean value that specifies whether the notification represents an error.</p>
    #[serde(rename = "error")]
    pub error: bool,
    /// <p>The event for which a notification is sent.</p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The message included in the notification.</p>
    #[serde(rename = "message")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// <p>The status code of the SNS notification.</p>
    #[serde(rename = "snsPublishStatusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_publish_status_code: Option<String>,
    /// <p>The SNS topic to which the SNS notification is sent.</p>
    #[serde(rename = "snsTopicArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sns_topic_arn: Option<String>,
}

/// <p>Used as one of the elements of the <a>AssessmentRun</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssessmentRunStateChange {
    /// <p>The assessment run state.</p>
    #[serde(rename = "state")]
    pub state: String,
    /// <p>The last time the assessment run state changed.</p>
    #[serde(rename = "stateChangedAt")]
    pub state_changed_at: f64,
}

/// <p>Contains information about an Amazon Inspector application. This data type is used as the response element in the <a>DescribeAssessmentTargets</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssessmentTarget {
    /// <p>The ARN that specifies the Amazon Inspector assessment target.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The time at which the assessment target is created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The name of the Amazon Inspector assessment target.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The ARN that specifies the resource group that is associated with the assessment target.</p>
    #[serde(rename = "resourceGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_arn: Option<String>,
    /// <p>The time at which <a>UpdateAssessmentTarget</a> is called.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
}

/// <p>Used as the request parameter in the <a>ListAssessmentTargets</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssessmentTargetFilter {
    /// <p>For a record to match a filter, an explicit value or a string that contains a wildcard that is specified for this data type property must match the value of the <b>assessmentTargetName</b> property of the <a>AssessmentTarget</a> data type.</p>
    #[serde(rename = "assessmentTargetNamePattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_target_name_pattern: Option<String>,
}

/// <p>Contains information about an Amazon Inspector assessment template. This data type is used as the response element in the <a>DescribeAssessmentTemplates</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssessmentTemplate {
    /// <p>The ARN of the assessment template.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The number of existing assessment runs associated with this assessment template. This value can be zero or a positive integer.</p>
    #[serde(rename = "assessmentRunCount")]
    pub assessment_run_count: i64,
    /// <p>The ARN of the assessment target that corresponds to this assessment template.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
    /// <p>The time at which the assessment template is created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The duration in seconds specified for this assessment template. The default value is 3600 seconds (one hour). The maximum value is 86400 seconds (one day).</p>
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i64,
    /// <p>The Amazon Resource Name (ARN) of the most recent assessment run associated with this assessment template. This value exists only when the value of assessmentRunCount is greaterpa than zero.</p>
    #[serde(rename = "lastAssessmentRunArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_assessment_run_arn: Option<String>,
    /// <p>The name of the assessment template.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The rules packages that are specified for this assessment template.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
    /// <p>The user-defined attributes that are assigned to every generated finding from the assessment run that uses this assessment template.</p>
    #[serde(rename = "userAttributesForFindings")]
    pub user_attributes_for_findings: Vec<Attribute>,
}

/// <p>Used as the request parameter in the <a>ListAssessmentTemplates</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct AssessmentTemplateFilter {
    /// <p>For a record to match a filter, the value specified for this data type property must inclusively match any value between the specified minimum and maximum values of the <b>durationInSeconds</b> property of the <a>AssessmentTemplate</a> data type.</p>
    #[serde(rename = "durationRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_range: Option<DurationRange>,
    /// <p>For a record to match a filter, an explicit value or a string that contains a wildcard that is specified for this data type property must match the value of the <b>assessmentTemplateName</b> property of the <a>AssessmentTemplate</a> data type.</p>
    #[serde(rename = "namePattern")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_pattern: Option<String>,
    /// <p>For a record to match a filter, the values that are specified for this data type property must be contained in the list of values of the <b>rulesPackageArns</b> property of the <a>AssessmentTemplate</a> data type.</p>
    #[serde(rename = "rulesPackageArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_package_arns: Option<Vec<String>>,
}

/// <p>A collection of attributes of the host from which the finding is generated.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct AssetAttributes {
    /// <p>The ID of the agent that is installed on the EC2 instance where the finding is generated.</p>
    #[serde(rename = "agentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// <p>The ID of the Amazon Machine Image (AMI) that is installed on the EC2 instance where the finding is generated.</p>
    #[serde(rename = "amiId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ami_id: Option<String>,
    /// <p>The Auto Scaling group of the EC2 instance where the finding is generated.</p>
    #[serde(rename = "autoScalingGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_group: Option<String>,
    /// <p>The hostname of the EC2 instance where the finding is generated.</p>
    #[serde(rename = "hostname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// <p>The list of IP v4 addresses of the EC2 instance where the finding is generated.</p>
    #[serde(rename = "ipv4Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_4_addresses: Option<Vec<String>>,
    /// <p>An array of the network interfaces interacting with the EC2 instance where the finding is generated.</p>
    #[serde(rename = "networkInterfaces")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interfaces: Option<Vec<NetworkInterface>>,
    /// <p>The schema version of this data type.</p>
    #[serde(rename = "schemaVersion")]
    pub schema_version: i64,
    /// <p>The tags related to the EC2 instance where the finding is generated.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

/// <p>This data type is used as a request parameter in the <a>AddAttributesToFindings</a> and <a>CreateAssessmentTemplate</a> actions.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Attribute {
    /// <p>The attribute key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value assigned to the attribute key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAssessmentTargetRequest {
    /// <p>The user-defined name that identifies the assessment target that you want to create. The name must be unique within the AWS account.</p>
    #[serde(rename = "assessmentTargetName")]
    pub assessment_target_name: String,
    /// <p>The ARN that specifies the resource group that is used to create the assessment target. If resourceGroupArn is not specified, all EC2 instances in the current AWS account and region are included in the assessment target.</p>
    #[serde(rename = "resourceGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAssessmentTargetResponse {
    /// <p>The ARN that specifies the assessment target that is created.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateAssessmentTemplateRequest {
    /// <p>The ARN that specifies the assessment target for which you want to create the assessment template.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
    /// <p>The user-defined name that identifies the assessment template that you want to create. You can create several assessment templates for an assessment target. The names of the assessment templates that correspond to a particular assessment target must be unique.</p>
    #[serde(rename = "assessmentTemplateName")]
    pub assessment_template_name: String,
    /// <p>The duration of the assessment run in seconds.</p>
    #[serde(rename = "durationInSeconds")]
    pub duration_in_seconds: i64,
    /// <p>The ARNs that specify the rules packages that you want to attach to the assessment template.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
    /// <p>The user-defined attributes that are assigned to every finding that is generated by the assessment run that uses this assessment template. An attribute is a key and value pair (an <a>Attribute</a> object). Within an assessment template, each key must be unique.</p>
    #[serde(rename = "userAttributesForFindings")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes_for_findings: Option<Vec<Attribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateAssessmentTemplateResponse {
    /// <p>The ARN that specifies the assessment template that is created.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateExclusionsPreviewRequest {
    /// <p>The ARN that specifies the assessment template for which you want to create an exclusions preview.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateExclusionsPreviewResponse {
    /// <p>Specifies the unique identifier of the requested exclusions preview. You can use the unique identifier to retrieve the exclusions preview when running the GetExclusionsPreview API.</p>
    #[serde(rename = "previewToken")]
    pub preview_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct CreateResourceGroupRequest {
    /// <p>A collection of keys and an array of possible values, '[{"key":"key1","values":["Value1","Value2"]},{"key":"Key2","values":["Value3"]}]'.</p> <p>For example,'[{"key":"Name","values":["TestEC2Instance"]}]'.</p>
    #[serde(rename = "resourceGroupTags")]
    pub resource_group_tags: Vec<ResourceGroupTag>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct CreateResourceGroupResponse {
    /// <p>The ARN that specifies the resource group that is created.</p>
    #[serde(rename = "resourceGroupArn")]
    pub resource_group_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAssessmentRunRequest {
    /// <p>The ARN that specifies the assessment run that you want to delete.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAssessmentTargetRequest {
    /// <p>The ARN that specifies the assessment target that you want to delete.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DeleteAssessmentTemplateRequest {
    /// <p>The ARN that specifies the assessment template that you want to delete.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssessmentRunsRequest {
    /// <p>The ARN that specifies the assessment run that you want to describe.</p>
    #[serde(rename = "assessmentRunArns")]
    pub assessment_run_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAssessmentRunsResponse {
    /// <p>Information about the assessment run.</p>
    #[serde(rename = "assessmentRuns")]
    pub assessment_runs: Vec<AssessmentRun>,
    /// <p>Assessment run details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssessmentTargetsRequest {
    /// <p>The ARNs that specifies the assessment targets that you want to describe.</p>
    #[serde(rename = "assessmentTargetArns")]
    pub assessment_target_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAssessmentTargetsResponse {
    /// <p>Information about the assessment targets.</p>
    #[serde(rename = "assessmentTargets")]
    pub assessment_targets: Vec<AssessmentTarget>,
    /// <p>Assessment target details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeAssessmentTemplatesRequest {
    #[serde(rename = "assessmentTemplateArns")]
    pub assessment_template_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeAssessmentTemplatesResponse {
    /// <p>Information about the assessment templates.</p>
    #[serde(rename = "assessmentTemplates")]
    pub assessment_templates: Vec<AssessmentTemplate>,
    /// <p>Assessment template details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeCrossAccountAccessRoleResponse {
    /// <p>The date when the cross-account access role was registered.</p>
    #[serde(rename = "registeredAt")]
    pub registered_at: f64,
    /// <p>The ARN that specifies the IAM role that Amazon Inspector uses to access your AWS account.</p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
    /// <p>A Boolean value that specifies whether the IAM role has the necessary policies attached to enable Amazon Inspector to access your AWS account.</p>
    #[serde(rename = "valid")]
    pub valid: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeExclusionsRequest {
    /// <p>The list of ARNs that specify the exclusions that you want to describe.</p>
    #[serde(rename = "exclusionArns")]
    pub exclusion_arns: Vec<String>,
    /// <p>The locale into which you want to translate the exclusion's title, description, and recommendation.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeExclusionsResponse {
    /// <p>Information about the exclusions.</p>
    #[serde(rename = "exclusions")]
    pub exclusions: ::std::collections::HashMap<String, Exclusion>,
    /// <p>Exclusion details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeFindingsRequest {
    /// <p>The ARN that specifies the finding that you want to describe.</p>
    #[serde(rename = "findingArns")]
    pub finding_arns: Vec<String>,
    /// <p>The locale into which you want to translate a finding description, recommendation, and the short description that identifies the finding.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeFindingsResponse {
    /// <p>Finding details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
    /// <p>Information about the finding.</p>
    #[serde(rename = "findings")]
    pub findings: Vec<Finding>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeResourceGroupsRequest {
    /// <p>The ARN that specifies the resource group that you want to describe.</p>
    #[serde(rename = "resourceGroupArns")]
    pub resource_group_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeResourceGroupsResponse {
    /// <p>Resource group details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
    /// <p>Information about a resource group.</p>
    #[serde(rename = "resourceGroups")]
    pub resource_groups: Vec<ResourceGroup>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DescribeRulesPackagesRequest {
    /// <p>The locale that you want to translate a rules package description into.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>The ARN that specifies the rules package that you want to describe.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct DescribeRulesPackagesResponse {
    /// <p>Rules package details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
    /// <p>Information about the rules package.</p>
    #[serde(rename = "rulesPackages")]
    pub rules_packages: Vec<RulesPackage>,
}

/// <p>This data type is used in the <a>AssessmentTemplateFilter</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct DurationRange {
    /// <p>The maximum value of the duration range. Must be less than or equal to 604800 seconds (1 week).</p>
    #[serde(rename = "maxSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_seconds: Option<i64>,
    /// <p>The minimum value of the duration range. Must be greater than zero.</p>
    #[serde(rename = "minSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_seconds: Option<i64>,
}

/// <p>This data type is used in the <a>Subscription</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct EventSubscription {
    /// <p>The event for which Amazon Simple Notification Service (SNS) notifications are sent.</p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The time at which <a>SubscribeToEvent</a> is called.</p>
    #[serde(rename = "subscribedAt")]
    pub subscribed_at: f64,
}

/// <p>Contains information about what was excluded from an assessment run.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Exclusion {
    /// <p>The ARN that specifies the exclusion.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The system-defined attributes for the exclusion.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The description of the exclusion.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The recommendation for the exclusion.</p>
    #[serde(rename = "recommendation")]
    pub recommendation: String,
    /// <p>The AWS resources for which the exclusion pertains.</p>
    #[serde(rename = "scopes")]
    pub scopes: Vec<Scope>,
    /// <p>The name of the exclusion.</p>
    #[serde(rename = "title")]
    pub title: String,
}

/// <p>Contains information about what is excluded from an assessment run given the current state of the assessment template.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ExclusionPreview {
    /// <p>The system-defined attributes for the exclusion preview.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>The description of the exclusion preview.</p>
    #[serde(rename = "description")]
    pub description: String,
    /// <p>The recommendation for the exclusion preview.</p>
    #[serde(rename = "recommendation")]
    pub recommendation: String,
    /// <p>The AWS resources for which the exclusion preview pertains.</p>
    #[serde(rename = "scopes")]
    pub scopes: Vec<Scope>,
    /// <p>The name of the exclusion preview.</p>
    #[serde(rename = "title")]
    pub title: String,
}

/// <p>Includes details about the failed items.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct FailedItemDetails {
    /// <p>The status code of a failed item.</p>
    #[serde(rename = "failureCode")]
    pub failure_code: String,
    /// <p>Indicates whether you can immediately retry a request for this item for a specified resource.</p>
    #[serde(rename = "retryable")]
    pub retryable: bool,
}

/// <p>Contains information about an Amazon Inspector finding. This data type is used as the response element in the <a>DescribeFindings</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Finding {
    /// <p>The ARN that specifies the finding.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>A collection of attributes of the host from which the finding is generated.</p>
    #[serde(rename = "assetAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_attributes: Option<AssetAttributes>,
    /// <p>The type of the host from which the finding is generated.</p>
    #[serde(rename = "assetType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    /// <p>The system-defined attributes for the finding.</p>
    #[serde(rename = "attributes")]
    pub attributes: Vec<Attribute>,
    /// <p>This data element is currently not used.</p>
    #[serde(rename = "confidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<i64>,
    /// <p>The time when the finding was generated.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The description of the finding.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The ID of the finding.</p>
    #[serde(rename = "id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// <p>This data element is currently not used.</p>
    #[serde(rename = "indicatorOfCompromise")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicator_of_compromise: Option<bool>,
    /// <p>The numeric value of the finding severity.</p>
    #[serde(rename = "numericSeverity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric_severity: Option<f64>,
    /// <p>The recommendation for the finding.</p>
    #[serde(rename = "recommendation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<String>,
    /// <p>The schema version of this data type.</p>
    #[serde(rename = "schemaVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_version: Option<i64>,
    /// <p>The data element is set to "Inspector".</p>
    #[serde(rename = "service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// <p>This data type is used in the <a>Finding</a> data type.</p>
    #[serde(rename = "serviceAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_attributes: Option<InspectorServiceAttributes>,
    /// <p>The finding severity. Values can be set to High, Medium, Low, and Informational.</p>
    #[serde(rename = "severity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
    /// <p>The name of the finding.</p>
    #[serde(rename = "title")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// <p>The time when <a>AddAttributesToFindings</a> is called.</p>
    #[serde(rename = "updatedAt")]
    pub updated_at: f64,
    /// <p>The user-defined attributes that are assigned to the finding.</p>
    #[serde(rename = "userAttributes")]
    pub user_attributes: Vec<Attribute>,
}

/// <p>This data type is used as a request parameter in the <a>ListFindings</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct FindingFilter {
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>agentId</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "agentIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_ids: Option<Vec<String>>,
    /// <p>For a record to match a filter, the list of values that are specified for this data type property must be contained in the list of values of the <b>attributes</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "attributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<Attribute>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>autoScalingGroup</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "autoScalingGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_scaling_groups: Option<Vec<String>>,
    /// <p>The time range during which the finding is generated.</p>
    #[serde(rename = "creationTimeRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creation_time_range: Option<TimestampRange>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>ruleName</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "ruleNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_names: Option<Vec<String>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>rulesPackageArn</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "rulesPackageArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_package_arns: Option<Vec<String>>,
    /// <p>For a record to match a filter, one of the values that is specified for this data type property must be the exact match of the value of the <b>severity</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "severities")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severities: Option<Vec<String>>,
    /// <p>For a record to match a filter, the value that is specified for this data type property must be contained in the list of values of the <b>userAttributes</b> property of the <a>Finding</a> data type.</p>
    #[serde(rename = "userAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_attributes: Option<Vec<Attribute>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetAssessmentReportRequest {
    /// <p>The ARN that specifies the assessment run for which you want to generate a report.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>Specifies the file format (html or pdf) of the assessment report that you want to generate.</p>
    #[serde(rename = "reportFileFormat")]
    pub report_file_format: String,
    /// <p>Specifies the type of the assessment report that you want to generate. There are two types of assessment reports: a finding report and a full report. For more information, see <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_reports.html">Assessment Reports</a>. </p>
    #[serde(rename = "reportType")]
    pub report_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetAssessmentReportResponse {
    /// <p>Specifies the status of the request to generate an assessment report. </p>
    #[serde(rename = "status")]
    pub status: String,
    /// <p>Specifies the URL where you can find the generated assessment report. This parameter is only returned if the report is successfully generated.</p>
    #[serde(rename = "url")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetExclusionsPreviewRequest {
    /// <p>The ARN that specifies the assessment template for which the exclusions preview was requested.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
    /// <p>The locale into which you want to translate the exclusion's title, description, and recommendation.</p>
    #[serde(rename = "locale")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 100. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the GetExclusionsPreviewRequest action. Subsequent calls to the action fill nextToken in the request with the value of nextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The unique identifier associated of the exclusions preview.</p>
    #[serde(rename = "previewToken")]
    pub preview_token: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetExclusionsPreviewResponse {
    /// <p>Information about the exclusions included in the preview.</p>
    #[serde(rename = "exclusionPreviews")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclusion_previews: Option<Vec<ExclusionPreview>>,
    /// <p>When a response is generated, if there is more data to be listed, this parameters is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Specifies the status of the request to generate an exclusions preview.</p>
    #[serde(rename = "previewStatus")]
    pub preview_status: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct GetTelemetryMetadataRequest {
    /// <p>The ARN that specifies the assessment run that has the telemetry data that you want to obtain.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct GetTelemetryMetadataResponse {
    /// <p>Telemetry details.</p>
    #[serde(rename = "telemetryMetadata")]
    pub telemetry_metadata: Vec<TelemetryMetadata>,
}

/// <p>This data type is used in the <a>Finding</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct InspectorServiceAttributes {
    /// <p>The ARN of the assessment run during which the finding is generated.</p>
    #[serde(rename = "assessmentRunArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_run_arn: Option<String>,
    /// <p>The ARN of the rules package that is used to generate the finding.</p>
    #[serde(rename = "rulesPackageArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rules_package_arn: Option<String>,
    /// <p>The schema version of this data type.</p>
    #[serde(rename = "schemaVersion")]
    pub schema_version: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssessmentRunAgentsRequest {
    /// <p>The ARN that specifies the assessment run whose agents you want to list.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AgentFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentRunAgents</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAssessmentRunAgentsResponse {
    /// <p>A list of ARNs that specifies the agents returned by the action.</p>
    #[serde(rename = "assessmentRunAgents")]
    pub assessment_run_agents: Vec<AssessmentRunAgent>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssessmentRunsRequest {
    /// <p>The ARNs that specify the assessment templates whose assessment runs you want to list.</p>
    #[serde(rename = "assessmentTemplateArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_template_arns: Option<Vec<String>>,
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AssessmentRunFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items that you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentRuns</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAssessmentRunsResponse {
    /// <p>A list of ARNs that specifies the assessment runs that are returned by the action.</p>
    #[serde(rename = "assessmentRunArns")]
    pub assessment_run_arns: Vec<String>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssessmentTargetsRequest {
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AssessmentTargetFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentTargets</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAssessmentTargetsResponse {
    /// <p>A list of ARNs that specifies the assessment targets that are returned by the action.</p>
    #[serde(rename = "assessmentTargetArns")]
    pub assessment_target_arns: Vec<String>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListAssessmentTemplatesRequest {
    /// <p>A list of ARNs that specifies the assessment targets whose assessment templates you want to list.</p>
    #[serde(rename = "assessmentTargetArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_target_arns: Option<Vec<String>>,
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<AssessmentTemplateFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListAssessmentTemplates</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListAssessmentTemplatesResponse {
    /// <p>A list of ARNs that specifies the assessment templates returned by the action.</p>
    #[serde(rename = "assessmentTemplateArns")]
    pub assessment_template_arns: Vec<String>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListEventSubscriptionsRequest {
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListEventSubscriptions</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the assessment template for which you want to list the existing event subscriptions.</p>
    #[serde(rename = "resourceArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_arn: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListEventSubscriptionsResponse {
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>Details of the returned event subscriptions.</p>
    #[serde(rename = "subscriptions")]
    pub subscriptions: Vec<Subscription>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListExclusionsRequest {
    /// <p>The ARN of the assessment run that generated the exclusions that you want to list.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 100. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the ListExclusionsRequest action. Subsequent calls to the action fill nextToken in the request with the value of nextToken from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListExclusionsResponse {
    /// <p>A list of exclusions' ARNs returned by the action.</p>
    #[serde(rename = "exclusionArns")]
    pub exclusion_arns: Vec<String>,
    /// <p>When a response is generated, if there is more data to be listed, this parameters is present in the response and contains the value to use for the nextToken parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListFindingsRequest {
    /// <p>The ARNs of the assessment runs that generate the findings that you want to list.</p>
    #[serde(rename = "assessmentRunArns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_run_arns: Option<Vec<String>>,
    /// <p>You can use this parameter to specify a subset of data to be included in the action's response.</p> <p>For a record to match a filter, all specified filter attributes must match. When multiple values are specified for a filter attribute, any of the values can match.</p>
    #[serde(rename = "filter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<FindingFilter>,
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListFindings</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListFindingsResponse {
    /// <p>A list of ARNs that specifies the findings returned by the action.</p>
    #[serde(rename = "findingArns")]
    pub finding_arns: Vec<String>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListRulesPackagesRequest {
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>ListRulesPackages</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListRulesPackagesResponse {
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The list of ARNs that specifies the rules packages returned by the action.</p>
    #[serde(rename = "rulesPackageArns")]
    pub rules_package_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ListTagsForResourceRequest {
    /// <p>The ARN that specifies the assessment template whose tags you want to list.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ListTagsForResourceResponse {
    /// <p>A collection of key and value pairs.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<Tag>,
}

/// <p>Contains information about the network interfaces interacting with an EC2 instance. This data type is used as one of the elements of the <a>AssetAttributes</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct NetworkInterface {
    /// <p>The IP addresses associated with the network interface.</p>
    #[serde(rename = "ipv6Addresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipv_6_addresses: Option<Vec<String>>,
    /// <p>The ID of the network interface.</p>
    #[serde(rename = "networkInterfaceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_interface_id: Option<String>,
    /// <p>The name of a private DNS associated with the network interface.</p>
    #[serde(rename = "privateDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// <p>The private IP address associated with the network interface.</p>
    #[serde(rename = "privateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
    /// <p>A list of the private IP addresses associated with the network interface. Includes the privateDnsName and privateIpAddress.</p>
    #[serde(rename = "privateIpAddresses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_addresses: Option<Vec<PrivateIp>>,
    /// <p>The name of a public DNS associated with the network interface.</p>
    #[serde(rename = "publicDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_dns_name: Option<String>,
    /// <p>The public IP address from which the network interface is reachable.</p>
    #[serde(rename = "publicIp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_ip: Option<String>,
    /// <p>A list of the security groups associated with the network interface. Includes the groupId and groupName.</p>
    #[serde(rename = "securityGroups")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_groups: Option<Vec<SecurityGroup>>,
    /// <p>The ID of a subnet associated with the network interface.</p>
    #[serde(rename = "subnetId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet_id: Option<String>,
    /// <p>The ID of a VPC associated with the network interface.</p>
    #[serde(rename = "vpcId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vpc_id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct PreviewAgentsRequest {
    /// <p>You can use this parameter to indicate the maximum number of items you want in the response. The default value is 10. The maximum value is 500.</p>
    #[serde(rename = "maxResults")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i64>,
    /// <p>You can use this parameter when paginating results. Set the value of this parameter to null on your first call to the <b>PreviewAgents</b> action. Subsequent calls to the action fill <b>nextToken</b> in the request with the value of <b>NextToken</b> from the previous response to continue listing data.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
    /// <p>The ARN of the assessment target whose agents you want to preview.</p>
    #[serde(rename = "previewAgentsArn")]
    pub preview_agents_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PreviewAgentsResponse {
    /// <p>The resulting list of agents.</p>
    #[serde(rename = "agentPreviews")]
    pub agent_previews: Vec<AgentPreview>,
    /// <p> When a response is generated, if there is more data to be listed, this parameter is present in the response and contains the value to use for the <b>nextToken</b> parameter in a subsequent pagination request. If there is no more data to be listed, this parameter is set to null.</p>
    #[serde(rename = "nextToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_token: Option<String>,
}

/// <p>Contains information about a private IP address associated with a network interface. This data type is used as a response element in the <a>DescribeFindings</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct PrivateIp {
    /// <p>The DNS name of the private IP address.</p>
    #[serde(rename = "privateDnsName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_dns_name: Option<String>,
    /// <p>The full IP address of the network inteface.</p>
    #[serde(rename = "privateIpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_ip_address: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RegisterCrossAccountAccessRoleRequest {
    /// <p>The ARN of the IAM role that grants Amazon Inspector access to AWS Services needed to perform security assessments. </p>
    #[serde(rename = "roleArn")]
    pub role_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct RemoveAttributesFromFindingsRequest {
    /// <p>The array of attribute keys that you want to remove from specified findings.</p>
    #[serde(rename = "attributeKeys")]
    pub attribute_keys: Vec<String>,
    /// <p>The ARNs that specify the findings that you want to remove attributes from.</p>
    #[serde(rename = "findingArns")]
    pub finding_arns: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RemoveAttributesFromFindingsResponse {
    /// <p>Attributes details that cannot be described. An error code is provided for each failed item.</p>
    #[serde(rename = "failedItems")]
    pub failed_items: ::std::collections::HashMap<String, FailedItemDetails>,
}

/// <p>Contains information about a resource group. The resource group defines a set of tags that, when queried, identify the AWS resources that make up the assessment target. This data type is used as the response element in the <a>DescribeResourceGroups</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct ResourceGroup {
    /// <p>The ARN of the resource group.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The time at which resource group is created.</p>
    #[serde(rename = "createdAt")]
    pub created_at: f64,
    /// <p>The tags (key and value pairs) of the resource group. This data type property is used in the <a>CreateResourceGroup</a> action.</p>
    #[serde(rename = "tags")]
    pub tags: Vec<ResourceGroupTag>,
}

/// <p>This data type is used as one of the elements of the <a>ResourceGroup</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResourceGroupTag {
    /// <p>A tag key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>The value assigned to a tag key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains information about an Amazon Inspector rules package. This data type is used as the response element in the <a>DescribeRulesPackages</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct RulesPackage {
    /// <p>The ARN of the rules package.</p>
    #[serde(rename = "arn")]
    pub arn: String,
    /// <p>The description of the rules package.</p>
    #[serde(rename = "description")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// <p>The name of the rules package.</p>
    #[serde(rename = "name")]
    pub name: String,
    /// <p>The provider of the rules package.</p>
    #[serde(rename = "provider")]
    pub provider: String,
    /// <p>The version ID of the rules package.</p>
    #[serde(rename = "version")]
    pub version: String,
}

/// <p>This data type contains key-value pairs that identify various Amazon resources.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Scope {
    /// <p>The type of the scope.</p>
    #[serde(rename = "key")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// <p>The resource identifier for the specified scope type.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>Contains information about a security group associated with a network interface. This data type is used as one of the elements of the <a>NetworkInterface</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct SecurityGroup {
    /// <p>The ID of the security group.</p>
    #[serde(rename = "groupId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// <p>The name of the security group.</p>
    #[serde(rename = "groupName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SetTagsForResourceRequest {
    /// <p>The ARN of the assessment template that you want to set tags to.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>A collection of key and value pairs that you want to set to the assessment template.</p>
    #[serde(rename = "tags")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<Tag>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StartAssessmentRunRequest {
    /// <p>You can specify the name for the assessment run. The name must be unique for the assessment template whose ARN is used to start the assessment run.</p>
    #[serde(rename = "assessmentRunName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assessment_run_name: Option<String>,
    /// <p>The ARN of the assessment template of the assessment run that you want to start.</p>
    #[serde(rename = "assessmentTemplateArn")]
    pub assessment_template_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct StartAssessmentRunResponse {
    /// <p>The ARN of the assessment run that has been started.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct StopAssessmentRunRequest {
    /// <p>The ARN of the assessment run that you want to stop.</p>
    #[serde(rename = "assessmentRunArn")]
    pub assessment_run_arn: String,
    /// <p>An input option that can be set to either START_EVALUATION or SKIP_EVALUATION. START_EVALUATION (the default value), stops the AWS agent from collecting data and begins the results evaluation and the findings generation process. SKIP_EVALUATION cancels the assessment run immediately, after which no findings are generated.</p>
    #[serde(rename = "stopAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_action: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct SubscribeToEventRequest {
    /// <p>The event for which you want to receive SNS notifications.</p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The ARN of the assessment template that is used during the event for which you want to receive SNS notifications.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The ARN of the SNS topic to which the SNS notifications are sent.</p>
    #[serde(rename = "topicArn")]
    pub topic_arn: String,
}

/// <p>This data type is used as a response element in the <a>ListEventSubscriptions</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct Subscription {
    /// <p>The list of existing event subscriptions.</p>
    #[serde(rename = "eventSubscriptions")]
    pub event_subscriptions: Vec<EventSubscription>,
    /// <p>The ARN of the assessment template that is used during the event for which the SNS notification is sent.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The ARN of the Amazon Simple Notification Service (SNS) topic to which the SNS notifications are sent.</p>
    #[serde(rename = "topicArn")]
    pub topic_arn: String,
}

/// <p>A key and value pair. This data type is used as a request parameter in the <a>SetTagsForResource</a> action and a response element in the <a>ListTagsForResource</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    /// <p>A tag key.</p>
    #[serde(rename = "key")]
    pub key: String,
    /// <p>A value assigned to a tag key.</p>
    #[serde(rename = "value")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// <p>The metadata about the Amazon Inspector application data metrics collected by the agent. This data type is used as the response element in the <a>GetTelemetryMetadata</a> action.</p>
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[cfg_attr(test, derive(Serialize))]
pub struct TelemetryMetadata {
    /// <p>The count of messages that the agent sends to the Amazon Inspector service.</p>
    #[serde(rename = "count")]
    pub count: i64,
    /// <p>The data size of messages that the agent sends to the Amazon Inspector service.</p>
    #[serde(rename = "dataSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_size: Option<i64>,
    /// <p>A specific type of behavioral data that is collected by the agent.</p>
    #[serde(rename = "messageType")]
    pub message_type: String,
}

/// <p>This data type is used in the <a>AssessmentRunFilter</a> data type.</p>
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct TimestampRange {
    /// <p>The minimum value of the timestamp range.</p>
    #[serde(rename = "beginDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub begin_date: Option<f64>,
    /// <p>The maximum value of the timestamp range.</p>
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<f64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UnsubscribeFromEventRequest {
    /// <p>The event for which you want to stop receiving SNS notifications.</p>
    #[serde(rename = "event")]
    pub event: String,
    /// <p>The ARN of the assessment template that is used during the event for which you want to stop receiving SNS notifications.</p>
    #[serde(rename = "resourceArn")]
    pub resource_arn: String,
    /// <p>The ARN of the SNS topic to which SNS notifications are sent.</p>
    #[serde(rename = "topicArn")]
    pub topic_arn: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct UpdateAssessmentTargetRequest {
    /// <p>The ARN of the assessment target that you want to update.</p>
    #[serde(rename = "assessmentTargetArn")]
    pub assessment_target_arn: String,
    /// <p>The name of the assessment target that you want to update.</p>
    #[serde(rename = "assessmentTargetName")]
    pub assessment_target_name: String,
    /// <p>The ARN of the resource group that is used to specify the new resource group to associate with the assessment target.</p>
    #[serde(rename = "resourceGroupArn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_group_arn: Option<String>,
}

/// Errors returned by AddAttributesToFindings
#[derive(Debug, PartialEq)]
pub enum AddAttributesToFindingsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl AddAttributesToFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<AddAttributesToFindingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(AddAttributesToFindingsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(AddAttributesToFindingsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(AddAttributesToFindingsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(AddAttributesToFindingsError::NoSuchEntity(
                        err.msg,
                    ))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        AddAttributesToFindingsError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for AddAttributesToFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for AddAttributesToFindingsError {
    fn description(&self) -> &str {
        match *self {
            AddAttributesToFindingsError::AccessDenied(ref cause) => cause,
            AddAttributesToFindingsError::Internal(ref cause) => cause,
            AddAttributesToFindingsError::InvalidInput(ref cause) => cause,
            AddAttributesToFindingsError::NoSuchEntity(ref cause) => cause,
            AddAttributesToFindingsError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAssessmentTarget
#[derive(Debug, PartialEq)]
pub enum CreateAssessmentTargetError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>Amazon Inspector cannot assume the cross-account role that it needs to list your EC2 instances during the assessment run.</p>
    InvalidCrossAccountRole(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl CreateAssessmentTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAssessmentTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateAssessmentTargetError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(CreateAssessmentTargetError::Internal(err.msg))
                }
                "InvalidCrossAccountRoleException" => {
                    return RusotoError::Service(
                        CreateAssessmentTargetError::InvalidCrossAccountRole(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateAssessmentTargetError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAssessmentTargetError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(CreateAssessmentTargetError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        CreateAssessmentTargetError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAssessmentTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAssessmentTargetError {
    fn description(&self) -> &str {
        match *self {
            CreateAssessmentTargetError::AccessDenied(ref cause) => cause,
            CreateAssessmentTargetError::Internal(ref cause) => cause,
            CreateAssessmentTargetError::InvalidCrossAccountRole(ref cause) => cause,
            CreateAssessmentTargetError::InvalidInput(ref cause) => cause,
            CreateAssessmentTargetError::LimitExceeded(ref cause) => cause,
            CreateAssessmentTargetError::NoSuchEntity(ref cause) => cause,
            CreateAssessmentTargetError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateAssessmentTemplate
#[derive(Debug, PartialEq)]
pub enum CreateAssessmentTemplateError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl CreateAssessmentTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateAssessmentTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateAssessmentTemplateError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(CreateAssessmentTemplateError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateAssessmentTemplateError::InvalidInput(
                        err.msg,
                    ))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateAssessmentTemplateError::LimitExceeded(
                        err.msg,
                    ))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(CreateAssessmentTemplateError::NoSuchEntity(
                        err.msg,
                    ))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        CreateAssessmentTemplateError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateAssessmentTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateAssessmentTemplateError {
    fn description(&self) -> &str {
        match *self {
            CreateAssessmentTemplateError::AccessDenied(ref cause) => cause,
            CreateAssessmentTemplateError::Internal(ref cause) => cause,
            CreateAssessmentTemplateError::InvalidInput(ref cause) => cause,
            CreateAssessmentTemplateError::LimitExceeded(ref cause) => cause,
            CreateAssessmentTemplateError::NoSuchEntity(ref cause) => cause,
            CreateAssessmentTemplateError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateExclusionsPreview
#[derive(Debug, PartialEq)]
pub enum CreateExclusionsPreviewError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The request is rejected. The specified assessment template is currently generating an exclusions preview.</p>
    PreviewGenerationInProgress(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl CreateExclusionsPreviewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateExclusionsPreviewError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateExclusionsPreviewError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(CreateExclusionsPreviewError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateExclusionsPreviewError::InvalidInput(
                        err.msg,
                    ))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(CreateExclusionsPreviewError::NoSuchEntity(
                        err.msg,
                    ))
                }
                "PreviewGenerationInProgressException" => {
                    return RusotoError::Service(
                        CreateExclusionsPreviewError::PreviewGenerationInProgress(err.msg),
                    )
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        CreateExclusionsPreviewError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateExclusionsPreviewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateExclusionsPreviewError {
    fn description(&self) -> &str {
        match *self {
            CreateExclusionsPreviewError::AccessDenied(ref cause) => cause,
            CreateExclusionsPreviewError::Internal(ref cause) => cause,
            CreateExclusionsPreviewError::InvalidInput(ref cause) => cause,
            CreateExclusionsPreviewError::NoSuchEntity(ref cause) => cause,
            CreateExclusionsPreviewError::PreviewGenerationInProgress(ref cause) => cause,
            CreateExclusionsPreviewError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by CreateResourceGroup
#[derive(Debug, PartialEq)]
pub enum CreateResourceGroupError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl CreateResourceGroupError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<CreateResourceGroupError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(CreateResourceGroupError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(CreateResourceGroupError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(CreateResourceGroupError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(CreateResourceGroupError::LimitExceeded(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        CreateResourceGroupError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for CreateResourceGroupError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for CreateResourceGroupError {
    fn description(&self) -> &str {
        match *self {
            CreateResourceGroupError::AccessDenied(ref cause) => cause,
            CreateResourceGroupError::Internal(ref cause) => cause,
            CreateResourceGroupError::InvalidInput(ref cause) => cause,
            CreateResourceGroupError::LimitExceeded(ref cause) => cause,
            CreateResourceGroupError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAssessmentRun
#[derive(Debug, PartialEq)]
pub enum DeleteAssessmentRunError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You cannot perform a specified action if an assessment run is currently in progress.</p>
    AssessmentRunInProgress(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl DeleteAssessmentRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAssessmentRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteAssessmentRunError::AccessDenied(err.msg))
                }
                "AssessmentRunInProgressException" => {
                    return RusotoError::Service(DeleteAssessmentRunError::AssessmentRunInProgress(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteAssessmentRunError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteAssessmentRunError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(DeleteAssessmentRunError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        DeleteAssessmentRunError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAssessmentRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAssessmentRunError {
    fn description(&self) -> &str {
        match *self {
            DeleteAssessmentRunError::AccessDenied(ref cause) => cause,
            DeleteAssessmentRunError::AssessmentRunInProgress(ref cause) => cause,
            DeleteAssessmentRunError::Internal(ref cause) => cause,
            DeleteAssessmentRunError::InvalidInput(ref cause) => cause,
            DeleteAssessmentRunError::NoSuchEntity(ref cause) => cause,
            DeleteAssessmentRunError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAssessmentTarget
#[derive(Debug, PartialEq)]
pub enum DeleteAssessmentTargetError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You cannot perform a specified action if an assessment run is currently in progress.</p>
    AssessmentRunInProgress(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl DeleteAssessmentTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAssessmentTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteAssessmentTargetError::AccessDenied(err.msg))
                }
                "AssessmentRunInProgressException" => {
                    return RusotoError::Service(
                        DeleteAssessmentTargetError::AssessmentRunInProgress(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteAssessmentTargetError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteAssessmentTargetError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(DeleteAssessmentTargetError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        DeleteAssessmentTargetError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAssessmentTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAssessmentTargetError {
    fn description(&self) -> &str {
        match *self {
            DeleteAssessmentTargetError::AccessDenied(ref cause) => cause,
            DeleteAssessmentTargetError::AssessmentRunInProgress(ref cause) => cause,
            DeleteAssessmentTargetError::Internal(ref cause) => cause,
            DeleteAssessmentTargetError::InvalidInput(ref cause) => cause,
            DeleteAssessmentTargetError::NoSuchEntity(ref cause) => cause,
            DeleteAssessmentTargetError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DeleteAssessmentTemplate
#[derive(Debug, PartialEq)]
pub enum DeleteAssessmentTemplateError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You cannot perform a specified action if an assessment run is currently in progress.</p>
    AssessmentRunInProgress(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl DeleteAssessmentTemplateError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DeleteAssessmentTemplateError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(DeleteAssessmentTemplateError::AccessDenied(
                        err.msg,
                    ))
                }
                "AssessmentRunInProgressException" => {
                    return RusotoError::Service(
                        DeleteAssessmentTemplateError::AssessmentRunInProgress(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(DeleteAssessmentTemplateError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DeleteAssessmentTemplateError::InvalidInput(
                        err.msg,
                    ))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(DeleteAssessmentTemplateError::NoSuchEntity(
                        err.msg,
                    ))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        DeleteAssessmentTemplateError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DeleteAssessmentTemplateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DeleteAssessmentTemplateError {
    fn description(&self) -> &str {
        match *self {
            DeleteAssessmentTemplateError::AccessDenied(ref cause) => cause,
            DeleteAssessmentTemplateError::AssessmentRunInProgress(ref cause) => cause,
            DeleteAssessmentTemplateError::Internal(ref cause) => cause,
            DeleteAssessmentTemplateError::InvalidInput(ref cause) => cause,
            DeleteAssessmentTemplateError::NoSuchEntity(ref cause) => cause,
            DeleteAssessmentTemplateError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssessmentRuns
#[derive(Debug, PartialEq)]
pub enum DescribeAssessmentRunsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeAssessmentRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAssessmentRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeAssessmentRunsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeAssessmentRunsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAssessmentRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssessmentRunsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssessmentRunsError::Internal(ref cause) => cause,
            DescribeAssessmentRunsError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssessmentTargets
#[derive(Debug, PartialEq)]
pub enum DescribeAssessmentTargetsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeAssessmentTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeAssessmentTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeAssessmentTargetsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeAssessmentTargetsError::InvalidInput(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAssessmentTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssessmentTargetsError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssessmentTargetsError::Internal(ref cause) => cause,
            DescribeAssessmentTargetsError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeAssessmentTemplates
#[derive(Debug, PartialEq)]
pub enum DescribeAssessmentTemplatesError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeAssessmentTemplatesError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeAssessmentTemplatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeAssessmentTemplatesError::Internal(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeAssessmentTemplatesError::InvalidInput(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeAssessmentTemplatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeAssessmentTemplatesError {
    fn description(&self) -> &str {
        match *self {
            DescribeAssessmentTemplatesError::Internal(ref cause) => cause,
            DescribeAssessmentTemplatesError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeCrossAccountAccessRole
#[derive(Debug, PartialEq)]
pub enum DescribeCrossAccountAccessRoleError {
    /// <p>Internal server error.</p>
    Internal(String),
}

impl DescribeCrossAccountAccessRoleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<DescribeCrossAccountAccessRoleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeCrossAccountAccessRoleError::Internal(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeCrossAccountAccessRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeCrossAccountAccessRoleError {
    fn description(&self) -> &str {
        match *self {
            DescribeCrossAccountAccessRoleError::Internal(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeExclusions
#[derive(Debug, PartialEq)]
pub enum DescribeExclusionsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeExclusionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeExclusionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeExclusionsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeExclusionsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeExclusionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeExclusionsError {
    fn description(&self) -> &str {
        match *self {
            DescribeExclusionsError::Internal(ref cause) => cause,
            DescribeExclusionsError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeFindings
#[derive(Debug, PartialEq)]
pub enum DescribeFindingsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeFindingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeFindingsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeFindingsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeFindingsError {
    fn description(&self) -> &str {
        match *self {
            DescribeFindingsError::Internal(ref cause) => cause,
            DescribeFindingsError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeResourceGroups
#[derive(Debug, PartialEq)]
pub enum DescribeResourceGroupsError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeResourceGroupsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeResourceGroupsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeResourceGroupsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeResourceGroupsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeResourceGroupsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeResourceGroupsError {
    fn description(&self) -> &str {
        match *self {
            DescribeResourceGroupsError::Internal(ref cause) => cause,
            DescribeResourceGroupsError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by DescribeRulesPackages
#[derive(Debug, PartialEq)]
pub enum DescribeRulesPackagesError {
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl DescribeRulesPackagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<DescribeRulesPackagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "InternalException" => {
                    return RusotoError::Service(DescribeRulesPackagesError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(DescribeRulesPackagesError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for DescribeRulesPackagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for DescribeRulesPackagesError {
    fn description(&self) -> &str {
        match *self {
            DescribeRulesPackagesError::Internal(ref cause) => cause,
            DescribeRulesPackagesError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by GetAssessmentReport
#[derive(Debug, PartialEq)]
pub enum GetAssessmentReportError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You cannot perform a specified action if an assessment run is currently in progress.</p>
    AssessmentRunInProgress(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
    /// <p>Used by the <a>GetAssessmentReport</a> API. The request was rejected because you tried to generate a report for an assessment run that existed before reporting was supported in Amazon Inspector. You can only generate reports for assessment runs that took place or will take place after generating reports in Amazon Inspector became available.</p>
    UnsupportedFeature(String),
}

impl GetAssessmentReportError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetAssessmentReportError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetAssessmentReportError::AccessDenied(err.msg))
                }
                "AssessmentRunInProgressException" => {
                    return RusotoError::Service(GetAssessmentReportError::AssessmentRunInProgress(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(GetAssessmentReportError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetAssessmentReportError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(GetAssessmentReportError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        GetAssessmentReportError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "UnsupportedFeatureException" => {
                    return RusotoError::Service(GetAssessmentReportError::UnsupportedFeature(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetAssessmentReportError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetAssessmentReportError {
    fn description(&self) -> &str {
        match *self {
            GetAssessmentReportError::AccessDenied(ref cause) => cause,
            GetAssessmentReportError::AssessmentRunInProgress(ref cause) => cause,
            GetAssessmentReportError::Internal(ref cause) => cause,
            GetAssessmentReportError::InvalidInput(ref cause) => cause,
            GetAssessmentReportError::NoSuchEntity(ref cause) => cause,
            GetAssessmentReportError::ServiceTemporarilyUnavailable(ref cause) => cause,
            GetAssessmentReportError::UnsupportedFeature(ref cause) => cause,
        }
    }
}
/// Errors returned by GetExclusionsPreview
#[derive(Debug, PartialEq)]
pub enum GetExclusionsPreviewError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl GetExclusionsPreviewError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetExclusionsPreviewError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetExclusionsPreviewError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(GetExclusionsPreviewError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetExclusionsPreviewError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(GetExclusionsPreviewError::NoSuchEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetExclusionsPreviewError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetExclusionsPreviewError {
    fn description(&self) -> &str {
        match *self {
            GetExclusionsPreviewError::AccessDenied(ref cause) => cause,
            GetExclusionsPreviewError::Internal(ref cause) => cause,
            GetExclusionsPreviewError::InvalidInput(ref cause) => cause,
            GetExclusionsPreviewError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by GetTelemetryMetadata
#[derive(Debug, PartialEq)]
pub enum GetTelemetryMetadataError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl GetTelemetryMetadataError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<GetTelemetryMetadataError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(GetTelemetryMetadataError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(GetTelemetryMetadataError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(GetTelemetryMetadataError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(GetTelemetryMetadataError::NoSuchEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for GetTelemetryMetadataError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for GetTelemetryMetadataError {
    fn description(&self) -> &str {
        match *self {
            GetTelemetryMetadataError::AccessDenied(ref cause) => cause,
            GetTelemetryMetadataError::Internal(ref cause) => cause,
            GetTelemetryMetadataError::InvalidInput(ref cause) => cause,
            GetTelemetryMetadataError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssessmentRunAgents
#[derive(Debug, PartialEq)]
pub enum ListAssessmentRunAgentsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl ListAssessmentRunAgentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssessmentRunAgentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAssessmentRunAgentsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(ListAssessmentRunAgentsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListAssessmentRunAgentsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(ListAssessmentRunAgentsError::NoSuchEntity(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssessmentRunAgentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssessmentRunAgentsError {
    fn description(&self) -> &str {
        match *self {
            ListAssessmentRunAgentsError::AccessDenied(ref cause) => cause,
            ListAssessmentRunAgentsError::Internal(ref cause) => cause,
            ListAssessmentRunAgentsError::InvalidInput(ref cause) => cause,
            ListAssessmentRunAgentsError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssessmentRuns
#[derive(Debug, PartialEq)]
pub enum ListAssessmentRunsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl ListAssessmentRunsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssessmentRunsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAssessmentRunsError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListAssessmentRunsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListAssessmentRunsError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(ListAssessmentRunsError::NoSuchEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssessmentRunsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssessmentRunsError {
    fn description(&self) -> &str {
        match *self {
            ListAssessmentRunsError::AccessDenied(ref cause) => cause,
            ListAssessmentRunsError::Internal(ref cause) => cause,
            ListAssessmentRunsError::InvalidInput(ref cause) => cause,
            ListAssessmentRunsError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssessmentTargets
#[derive(Debug, PartialEq)]
pub enum ListAssessmentTargetsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl ListAssessmentTargetsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssessmentTargetsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAssessmentTargetsError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListAssessmentTargetsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListAssessmentTargetsError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssessmentTargetsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssessmentTargetsError {
    fn description(&self) -> &str {
        match *self {
            ListAssessmentTargetsError::AccessDenied(ref cause) => cause,
            ListAssessmentTargetsError::Internal(ref cause) => cause,
            ListAssessmentTargetsError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by ListAssessmentTemplates
#[derive(Debug, PartialEq)]
pub enum ListAssessmentTemplatesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl ListAssessmentTemplatesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListAssessmentTemplatesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListAssessmentTemplatesError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(ListAssessmentTemplatesError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListAssessmentTemplatesError::InvalidInput(
                        err.msg,
                    ))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(ListAssessmentTemplatesError::NoSuchEntity(
                        err.msg,
                    ))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListAssessmentTemplatesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListAssessmentTemplatesError {
    fn description(&self) -> &str {
        match *self {
            ListAssessmentTemplatesError::AccessDenied(ref cause) => cause,
            ListAssessmentTemplatesError::Internal(ref cause) => cause,
            ListAssessmentTemplatesError::InvalidInput(ref cause) => cause,
            ListAssessmentTemplatesError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListEventSubscriptions
#[derive(Debug, PartialEq)]
pub enum ListEventSubscriptionsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl ListEventSubscriptionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListEventSubscriptionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListEventSubscriptionsError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListEventSubscriptionsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListEventSubscriptionsError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(ListEventSubscriptionsError::NoSuchEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListEventSubscriptionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListEventSubscriptionsError {
    fn description(&self) -> &str {
        match *self {
            ListEventSubscriptionsError::AccessDenied(ref cause) => cause,
            ListEventSubscriptionsError::Internal(ref cause) => cause,
            ListEventSubscriptionsError::InvalidInput(ref cause) => cause,
            ListEventSubscriptionsError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListExclusions
#[derive(Debug, PartialEq)]
pub enum ListExclusionsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl ListExclusionsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListExclusionsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListExclusionsError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListExclusionsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListExclusionsError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(ListExclusionsError::NoSuchEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListExclusionsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListExclusionsError {
    fn description(&self) -> &str {
        match *self {
            ListExclusionsError::AccessDenied(ref cause) => cause,
            ListExclusionsError::Internal(ref cause) => cause,
            ListExclusionsError::InvalidInput(ref cause) => cause,
            ListExclusionsError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListFindings
#[derive(Debug, PartialEq)]
pub enum ListFindingsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl ListFindingsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListFindingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListFindingsError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListFindingsError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListFindingsError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(ListFindingsError::NoSuchEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListFindingsError {
    fn description(&self) -> &str {
        match *self {
            ListFindingsError::AccessDenied(ref cause) => cause,
            ListFindingsError::Internal(ref cause) => cause,
            ListFindingsError::InvalidInput(ref cause) => cause,
            ListFindingsError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by ListRulesPackages
#[derive(Debug, PartialEq)]
pub enum ListRulesPackagesError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
}

impl ListRulesPackagesError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListRulesPackagesError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListRulesPackagesError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListRulesPackagesError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListRulesPackagesError::InvalidInput(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListRulesPackagesError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListRulesPackagesError {
    fn description(&self) -> &str {
        match *self {
            ListRulesPackagesError::AccessDenied(ref cause) => cause,
            ListRulesPackagesError::Internal(ref cause) => cause,
            ListRulesPackagesError::InvalidInput(ref cause) => cause,
        }
    }
}
/// Errors returned by ListTagsForResource
#[derive(Debug, PartialEq)]
pub enum ListTagsForResourceError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl ListTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<ListTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(ListTagsForResourceError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(ListTagsForResourceError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(ListTagsForResourceError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(ListTagsForResourceError::NoSuchEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for ListTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for ListTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            ListTagsForResourceError::AccessDenied(ref cause) => cause,
            ListTagsForResourceError::Internal(ref cause) => cause,
            ListTagsForResourceError::InvalidInput(ref cause) => cause,
            ListTagsForResourceError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by PreviewAgents
#[derive(Debug, PartialEq)]
pub enum PreviewAgentsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>Amazon Inspector cannot assume the cross-account role that it needs to list your EC2 instances during the assessment run.</p>
    InvalidCrossAccountRole(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
}

impl PreviewAgentsError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<PreviewAgentsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(PreviewAgentsError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(PreviewAgentsError::Internal(err.msg))
                }
                "InvalidCrossAccountRoleException" => {
                    return RusotoError::Service(PreviewAgentsError::InvalidCrossAccountRole(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(PreviewAgentsError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(PreviewAgentsError::NoSuchEntity(err.msg))
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for PreviewAgentsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for PreviewAgentsError {
    fn description(&self) -> &str {
        match *self {
            PreviewAgentsError::AccessDenied(ref cause) => cause,
            PreviewAgentsError::Internal(ref cause) => cause,
            PreviewAgentsError::InvalidCrossAccountRole(ref cause) => cause,
            PreviewAgentsError::InvalidInput(ref cause) => cause,
            PreviewAgentsError::NoSuchEntity(ref cause) => cause,
        }
    }
}
/// Errors returned by RegisterCrossAccountAccessRole
#[derive(Debug, PartialEq)]
pub enum RegisterCrossAccountAccessRoleError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>Amazon Inspector cannot assume the cross-account role that it needs to list your EC2 instances during the assessment run.</p>
    InvalidCrossAccountRole(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl RegisterCrossAccountAccessRoleError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RegisterCrossAccountAccessRoleError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RegisterCrossAccountAccessRoleError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(RegisterCrossAccountAccessRoleError::Internal(
                        err.msg,
                    ))
                }
                "InvalidCrossAccountRoleException" => {
                    return RusotoError::Service(
                        RegisterCrossAccountAccessRoleError::InvalidCrossAccountRole(err.msg),
                    )
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RegisterCrossAccountAccessRoleError::InvalidInput(
                        err.msg,
                    ))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        RegisterCrossAccountAccessRoleError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RegisterCrossAccountAccessRoleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RegisterCrossAccountAccessRoleError {
    fn description(&self) -> &str {
        match *self {
            RegisterCrossAccountAccessRoleError::AccessDenied(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::Internal(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::InvalidCrossAccountRole(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::InvalidInput(ref cause) => cause,
            RegisterCrossAccountAccessRoleError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by RemoveAttributesFromFindings
#[derive(Debug, PartialEq)]
pub enum RemoveAttributesFromFindingsError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl RemoveAttributesFromFindingsError {
    pub fn from_response(
        res: BufferedHttpResponse,
    ) -> RusotoError<RemoveAttributesFromFindingsError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(RemoveAttributesFromFindingsError::AccessDenied(
                        err.msg,
                    ))
                }
                "InternalException" => {
                    return RusotoError::Service(RemoveAttributesFromFindingsError::Internal(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(RemoveAttributesFromFindingsError::InvalidInput(
                        err.msg,
                    ))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(RemoveAttributesFromFindingsError::NoSuchEntity(
                        err.msg,
                    ))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        RemoveAttributesFromFindingsError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for RemoveAttributesFromFindingsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for RemoveAttributesFromFindingsError {
    fn description(&self) -> &str {
        match *self {
            RemoveAttributesFromFindingsError::AccessDenied(ref cause) => cause,
            RemoveAttributesFromFindingsError::Internal(ref cause) => cause,
            RemoveAttributesFromFindingsError::InvalidInput(ref cause) => cause,
            RemoveAttributesFromFindingsError::NoSuchEntity(ref cause) => cause,
            RemoveAttributesFromFindingsError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by SetTagsForResource
#[derive(Debug, PartialEq)]
pub enum SetTagsForResourceError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl SetTagsForResourceError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SetTagsForResourceError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SetTagsForResourceError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(SetTagsForResourceError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(SetTagsForResourceError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(SetTagsForResourceError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        SetTagsForResourceError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SetTagsForResourceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SetTagsForResourceError {
    fn description(&self) -> &str {
        match *self {
            SetTagsForResourceError::AccessDenied(ref cause) => cause,
            SetTagsForResourceError::Internal(ref cause) => cause,
            SetTagsForResourceError::InvalidInput(ref cause) => cause,
            SetTagsForResourceError::NoSuchEntity(ref cause) => cause,
            SetTagsForResourceError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by StartAssessmentRun
#[derive(Debug, PartialEq)]
pub enum StartAssessmentRunError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>You started an assessment run, but one of the instances is already participating in another assessment run.</p>
    AgentsAlreadyRunningAssessment(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>Amazon Inspector cannot assume the cross-account role that it needs to list your EC2 instances during the assessment run.</p>
    InvalidCrossAccountRole(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl StartAssessmentRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StartAssessmentRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StartAssessmentRunError::AccessDenied(err.msg))
                }
                "AgentsAlreadyRunningAssessmentException" => {
                    return RusotoError::Service(
                        StartAssessmentRunError::AgentsAlreadyRunningAssessment(err.msg),
                    )
                }
                "InternalException" => {
                    return RusotoError::Service(StartAssessmentRunError::Internal(err.msg))
                }
                "InvalidCrossAccountRoleException" => {
                    return RusotoError::Service(StartAssessmentRunError::InvalidCrossAccountRole(
                        err.msg,
                    ))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StartAssessmentRunError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(StartAssessmentRunError::LimitExceeded(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(StartAssessmentRunError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        StartAssessmentRunError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StartAssessmentRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StartAssessmentRunError {
    fn description(&self) -> &str {
        match *self {
            StartAssessmentRunError::AccessDenied(ref cause) => cause,
            StartAssessmentRunError::AgentsAlreadyRunningAssessment(ref cause) => cause,
            StartAssessmentRunError::Internal(ref cause) => cause,
            StartAssessmentRunError::InvalidCrossAccountRole(ref cause) => cause,
            StartAssessmentRunError::InvalidInput(ref cause) => cause,
            StartAssessmentRunError::LimitExceeded(ref cause) => cause,
            StartAssessmentRunError::NoSuchEntity(ref cause) => cause,
            StartAssessmentRunError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by StopAssessmentRun
#[derive(Debug, PartialEq)]
pub enum StopAssessmentRunError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl StopAssessmentRunError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<StopAssessmentRunError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(StopAssessmentRunError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(StopAssessmentRunError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(StopAssessmentRunError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(StopAssessmentRunError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        StopAssessmentRunError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for StopAssessmentRunError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for StopAssessmentRunError {
    fn description(&self) -> &str {
        match *self {
            StopAssessmentRunError::AccessDenied(ref cause) => cause,
            StopAssessmentRunError::Internal(ref cause) => cause,
            StopAssessmentRunError::InvalidInput(ref cause) => cause,
            StopAssessmentRunError::NoSuchEntity(ref cause) => cause,
            StopAssessmentRunError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by SubscribeToEvent
#[derive(Debug, PartialEq)]
pub enum SubscribeToEventError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it attempted to create resources beyond the current AWS account limits. The error code describes the limit exceeded.</p>
    LimitExceeded(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl SubscribeToEventError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<SubscribeToEventError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(SubscribeToEventError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(SubscribeToEventError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(SubscribeToEventError::InvalidInput(err.msg))
                }
                "LimitExceededException" => {
                    return RusotoError::Service(SubscribeToEventError::LimitExceeded(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(SubscribeToEventError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        SubscribeToEventError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for SubscribeToEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for SubscribeToEventError {
    fn description(&self) -> &str {
        match *self {
            SubscribeToEventError::AccessDenied(ref cause) => cause,
            SubscribeToEventError::Internal(ref cause) => cause,
            SubscribeToEventError::InvalidInput(ref cause) => cause,
            SubscribeToEventError::LimitExceeded(ref cause) => cause,
            SubscribeToEventError::NoSuchEntity(ref cause) => cause,
            SubscribeToEventError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by UnsubscribeFromEvent
#[derive(Debug, PartialEq)]
pub enum UnsubscribeFromEventError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl UnsubscribeFromEventError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UnsubscribeFromEventError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UnsubscribeFromEventError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(UnsubscribeFromEventError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UnsubscribeFromEventError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(UnsubscribeFromEventError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        UnsubscribeFromEventError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UnsubscribeFromEventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UnsubscribeFromEventError {
    fn description(&self) -> &str {
        match *self {
            UnsubscribeFromEventError::AccessDenied(ref cause) => cause,
            UnsubscribeFromEventError::Internal(ref cause) => cause,
            UnsubscribeFromEventError::InvalidInput(ref cause) => cause,
            UnsubscribeFromEventError::NoSuchEntity(ref cause) => cause,
            UnsubscribeFromEventError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Errors returned by UpdateAssessmentTarget
#[derive(Debug, PartialEq)]
pub enum UpdateAssessmentTargetError {
    /// <p>You do not have required permissions to access the requested resource.</p>
    AccessDenied(String),
    /// <p>Internal server error.</p>
    Internal(String),
    /// <p>The request was rejected because an invalid or out-of-range value was supplied for an input parameter.</p>
    InvalidInput(String),
    /// <p>The request was rejected because it referenced an entity that does not exist. The error code describes the entity.</p>
    NoSuchEntity(String),
    /// <p>The serice is temporary unavailable.</p>
    ServiceTemporarilyUnavailable(String),
}

impl UpdateAssessmentTargetError {
    pub fn from_response(res: BufferedHttpResponse) -> RusotoError<UpdateAssessmentTargetError> {
        if let Some(err) = proto::json::Error::parse(&res) {
            match err.typ.as_str() {
                "AccessDeniedException" => {
                    return RusotoError::Service(UpdateAssessmentTargetError::AccessDenied(err.msg))
                }
                "InternalException" => {
                    return RusotoError::Service(UpdateAssessmentTargetError::Internal(err.msg))
                }
                "InvalidInputException" => {
                    return RusotoError::Service(UpdateAssessmentTargetError::InvalidInput(err.msg))
                }
                "NoSuchEntityException" => {
                    return RusotoError::Service(UpdateAssessmentTargetError::NoSuchEntity(err.msg))
                }
                "ServiceTemporarilyUnavailableException" => {
                    return RusotoError::Service(
                        UpdateAssessmentTargetError::ServiceTemporarilyUnavailable(err.msg),
                    )
                }
                "ValidationException" => return RusotoError::Validation(err.msg),
                _ => {}
            }
        }
        return RusotoError::Unknown(res);
    }
}
impl fmt::Display for UpdateAssessmentTargetError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}
impl Error for UpdateAssessmentTargetError {
    fn description(&self) -> &str {
        match *self {
            UpdateAssessmentTargetError::AccessDenied(ref cause) => cause,
            UpdateAssessmentTargetError::Internal(ref cause) => cause,
            UpdateAssessmentTargetError::InvalidInput(ref cause) => cause,
            UpdateAssessmentTargetError::NoSuchEntity(ref cause) => cause,
            UpdateAssessmentTargetError::ServiceTemporarilyUnavailable(ref cause) => cause,
        }
    }
}
/// Trait representing the capabilities of the Amazon Inspector API. Amazon Inspector clients implement this trait.
pub trait Inspector {
    /// <p>Assigns attributes (key and value pairs) to the findings that are specified by the ARNs of the findings.</p>
    fn add_attributes_to_findings(
        &self,
        input: AddAttributesToFindingsRequest,
    ) -> RusotoFuture<AddAttributesToFindingsResponse, AddAttributesToFindingsError>;

    /// <p>Creates a new assessment target using the ARN of the resource group that is generated by <a>CreateResourceGroup</a>. If resourceGroupArn is not specified, all EC2 instances in the current AWS account and region are included in the assessment target. If the <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_slr.html">service-linked role</a> isn’t already registered, this action also creates and registers a service-linked role to grant Amazon Inspector access to AWS Services needed to perform security assessments. You can create up to 50 assessment targets per AWS account. You can run up to 500 concurrent agents per AWS account. For more information, see <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_applications.html"> Amazon Inspector Assessment Targets</a>.</p>
    fn create_assessment_target(
        &self,
        input: CreateAssessmentTargetRequest,
    ) -> RusotoFuture<CreateAssessmentTargetResponse, CreateAssessmentTargetError>;

    /// <p>Creates an assessment template for the assessment target that is specified by the ARN of the assessment target. If the <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_slr.html">service-linked role</a> isn’t already registered, this action also creates and registers a service-linked role to grant Amazon Inspector access to AWS Services needed to perform security assessments.</p>
    fn create_assessment_template(
        &self,
        input: CreateAssessmentTemplateRequest,
    ) -> RusotoFuture<CreateAssessmentTemplateResponse, CreateAssessmentTemplateError>;

    /// <p>Starts the generation of an exclusions preview for the specified assessment template. The exclusions preview lists the potential exclusions (ExclusionPreview) that Inspector can detect before it runs the assessment. </p>
    fn create_exclusions_preview(
        &self,
        input: CreateExclusionsPreviewRequest,
    ) -> RusotoFuture<CreateExclusionsPreviewResponse, CreateExclusionsPreviewError>;

    /// <p>Creates a resource group using the specified set of tags (key and value pairs) that are used to select the EC2 instances to be included in an Amazon Inspector assessment target. The created resource group is then used to create an Amazon Inspector assessment target. For more information, see <a>CreateAssessmentTarget</a>.</p>
    fn create_resource_group(
        &self,
        input: CreateResourceGroupRequest,
    ) -> RusotoFuture<CreateResourceGroupResponse, CreateResourceGroupError>;

    /// <p>Deletes the assessment run that is specified by the ARN of the assessment run.</p>
    fn delete_assessment_run(
        &self,
        input: DeleteAssessmentRunRequest,
    ) -> RusotoFuture<(), DeleteAssessmentRunError>;

    /// <p>Deletes the assessment target that is specified by the ARN of the assessment target.</p>
    fn delete_assessment_target(
        &self,
        input: DeleteAssessmentTargetRequest,
    ) -> RusotoFuture<(), DeleteAssessmentTargetError>;

    /// <p>Deletes the assessment template that is specified by the ARN of the assessment template.</p>
    fn delete_assessment_template(
        &self,
        input: DeleteAssessmentTemplateRequest,
    ) -> RusotoFuture<(), DeleteAssessmentTemplateError>;

    /// <p>Describes the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn describe_assessment_runs(
        &self,
        input: DescribeAssessmentRunsRequest,
    ) -> RusotoFuture<DescribeAssessmentRunsResponse, DescribeAssessmentRunsError>;

    /// <p>Describes the assessment targets that are specified by the ARNs of the assessment targets.</p>
    fn describe_assessment_targets(
        &self,
        input: DescribeAssessmentTargetsRequest,
    ) -> RusotoFuture<DescribeAssessmentTargetsResponse, DescribeAssessmentTargetsError>;

    /// <p>Describes the assessment templates that are specified by the ARNs of the assessment templates.</p>
    fn describe_assessment_templates(
        &self,
        input: DescribeAssessmentTemplatesRequest,
    ) -> RusotoFuture<DescribeAssessmentTemplatesResponse, DescribeAssessmentTemplatesError>;

    /// <p>Describes the IAM role that enables Amazon Inspector to access your AWS account.</p>
    fn describe_cross_account_access_role(
        &self,
    ) -> RusotoFuture<DescribeCrossAccountAccessRoleResponse, DescribeCrossAccountAccessRoleError>;

    /// <p>Describes the exclusions that are specified by the exclusions' ARNs.</p>
    fn describe_exclusions(
        &self,
        input: DescribeExclusionsRequest,
    ) -> RusotoFuture<DescribeExclusionsResponse, DescribeExclusionsError>;

    /// <p>Describes the findings that are specified by the ARNs of the findings.</p>
    fn describe_findings(
        &self,
        input: DescribeFindingsRequest,
    ) -> RusotoFuture<DescribeFindingsResponse, DescribeFindingsError>;

    /// <p>Describes the resource groups that are specified by the ARNs of the resource groups.</p>
    fn describe_resource_groups(
        &self,
        input: DescribeResourceGroupsRequest,
    ) -> RusotoFuture<DescribeResourceGroupsResponse, DescribeResourceGroupsError>;

    /// <p>Describes the rules packages that are specified by the ARNs of the rules packages.</p>
    fn describe_rules_packages(
        &self,
        input: DescribeRulesPackagesRequest,
    ) -> RusotoFuture<DescribeRulesPackagesResponse, DescribeRulesPackagesError>;

    /// <p>Produces an assessment report that includes detailed and comprehensive results of a specified assessment run. </p>
    fn get_assessment_report(
        &self,
        input: GetAssessmentReportRequest,
    ) -> RusotoFuture<GetAssessmentReportResponse, GetAssessmentReportError>;

    /// <p>Retrieves the exclusions preview (a list of ExclusionPreview objects) specified by the preview token. You can obtain the preview token by running the CreateExclusionsPreview API.</p>
    fn get_exclusions_preview(
        &self,
        input: GetExclusionsPreviewRequest,
    ) -> RusotoFuture<GetExclusionsPreviewResponse, GetExclusionsPreviewError>;

    /// <p>Information about the data that is collected for the specified assessment run.</p>
    fn get_telemetry_metadata(
        &self,
        input: GetTelemetryMetadataRequest,
    ) -> RusotoFuture<GetTelemetryMetadataResponse, GetTelemetryMetadataError>;

    /// <p>Lists the agents of the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn list_assessment_run_agents(
        &self,
        input: ListAssessmentRunAgentsRequest,
    ) -> RusotoFuture<ListAssessmentRunAgentsResponse, ListAssessmentRunAgentsError>;

    /// <p>Lists the assessment runs that correspond to the assessment templates that are specified by the ARNs of the assessment templates.</p>
    fn list_assessment_runs(
        &self,
        input: ListAssessmentRunsRequest,
    ) -> RusotoFuture<ListAssessmentRunsResponse, ListAssessmentRunsError>;

    /// <p>Lists the ARNs of the assessment targets within this AWS account. For more information about assessment targets, see <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_applications.html">Amazon Inspector Assessment Targets</a>.</p>
    fn list_assessment_targets(
        &self,
        input: ListAssessmentTargetsRequest,
    ) -> RusotoFuture<ListAssessmentTargetsResponse, ListAssessmentTargetsError>;

    /// <p>Lists the assessment templates that correspond to the assessment targets that are specified by the ARNs of the assessment targets.</p>
    fn list_assessment_templates(
        &self,
        input: ListAssessmentTemplatesRequest,
    ) -> RusotoFuture<ListAssessmentTemplatesResponse, ListAssessmentTemplatesError>;

    /// <p>Lists all the event subscriptions for the assessment template that is specified by the ARN of the assessment template. For more information, see <a>SubscribeToEvent</a> and <a>UnsubscribeFromEvent</a>.</p>
    fn list_event_subscriptions(
        &self,
        input: ListEventSubscriptionsRequest,
    ) -> RusotoFuture<ListEventSubscriptionsResponse, ListEventSubscriptionsError>;

    /// <p>List exclusions that are generated by the assessment run.</p>
    fn list_exclusions(
        &self,
        input: ListExclusionsRequest,
    ) -> RusotoFuture<ListExclusionsResponse, ListExclusionsError>;

    /// <p>Lists findings that are generated by the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> RusotoFuture<ListFindingsResponse, ListFindingsError>;

    /// <p>Lists all available Amazon Inspector rules packages.</p>
    fn list_rules_packages(
        &self,
        input: ListRulesPackagesRequest,
    ) -> RusotoFuture<ListRulesPackagesResponse, ListRulesPackagesError>;

    /// <p>Lists all tags associated with an assessment template.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError>;

    /// <p>Previews the agents installed on the EC2 instances that are part of the specified assessment target.</p>
    fn preview_agents(
        &self,
        input: PreviewAgentsRequest,
    ) -> RusotoFuture<PreviewAgentsResponse, PreviewAgentsError>;

    /// <p>Registers the IAM role that grants Amazon Inspector access to AWS Services needed to perform security assessments.</p>
    fn register_cross_account_access_role(
        &self,
        input: RegisterCrossAccountAccessRoleRequest,
    ) -> RusotoFuture<(), RegisterCrossAccountAccessRoleError>;

    /// <p>Removes entire attributes (key and value pairs) from the findings that are specified by the ARNs of the findings where an attribute with the specified key exists.</p>
    fn remove_attributes_from_findings(
        &self,
        input: RemoveAttributesFromFindingsRequest,
    ) -> RusotoFuture<RemoveAttributesFromFindingsResponse, RemoveAttributesFromFindingsError>;

    /// <p>Sets tags (key and value pairs) to the assessment template that is specified by the ARN of the assessment template.</p>
    fn set_tags_for_resource(
        &self,
        input: SetTagsForResourceRequest,
    ) -> RusotoFuture<(), SetTagsForResourceError>;

    /// <p>Starts the assessment run specified by the ARN of the assessment template. For this API to function properly, you must not exceed the limit of running up to 500 concurrent agents per AWS account.</p>
    fn start_assessment_run(
        &self,
        input: StartAssessmentRunRequest,
    ) -> RusotoFuture<StartAssessmentRunResponse, StartAssessmentRunError>;

    /// <p>Stops the assessment run that is specified by the ARN of the assessment run.</p>
    fn stop_assessment_run(
        &self,
        input: StopAssessmentRunRequest,
    ) -> RusotoFuture<(), StopAssessmentRunError>;

    /// <p>Enables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic.</p>
    fn subscribe_to_event(
        &self,
        input: SubscribeToEventRequest,
    ) -> RusotoFuture<(), SubscribeToEventError>;

    /// <p>Disables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic.</p>
    fn unsubscribe_from_event(
        &self,
        input: UnsubscribeFromEventRequest,
    ) -> RusotoFuture<(), UnsubscribeFromEventError>;

    /// <p>Updates the assessment target that is specified by the ARN of the assessment target.</p> <p>If resourceGroupArn is not specified, all EC2 instances in the current AWS account and region are included in the assessment target.</p>
    fn update_assessment_target(
        &self,
        input: UpdateAssessmentTargetRequest,
    ) -> RusotoFuture<(), UpdateAssessmentTargetError>;
}
/// A client for the Amazon Inspector API.
#[derive(Clone)]
pub struct InspectorClient {
    client: Client,
    region: region::Region,
}

impl InspectorClient {
    /// Creates a client backed by the default tokio event loop.
    ///
    /// The client will use the default credentials provider and tls client.
    pub fn new(region: region::Region) -> InspectorClient {
        InspectorClient {
            client: Client::shared(),
            region: region,
        }
    }

    pub fn new_with<P, D>(
        request_dispatcher: D,
        credentials_provider: P,
        region: region::Region,
    ) -> InspectorClient
    where
        P: ProvideAwsCredentials + Send + Sync + 'static,
        P::Future: Send,
        D: DispatchSignedRequest + Send + Sync + 'static,
        D::Future: Send,
    {
        InspectorClient {
            client: Client::new_with(credentials_provider, request_dispatcher),
            region: region,
        }
    }
}

impl Inspector for InspectorClient {
    /// <p>Assigns attributes (key and value pairs) to the findings that are specified by the ARNs of the findings.</p>
    fn add_attributes_to_findings(
        &self,
        input: AddAttributesToFindingsRequest,
    ) -> RusotoFuture<AddAttributesToFindingsResponse, AddAttributesToFindingsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.AddAttributesToFindings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<AddAttributesToFindingsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(AddAttributesToFindingsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a new assessment target using the ARN of the resource group that is generated by <a>CreateResourceGroup</a>. If resourceGroupArn is not specified, all EC2 instances in the current AWS account and region are included in the assessment target. If the <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_slr.html">service-linked role</a> isn’t already registered, this action also creates and registers a service-linked role to grant Amazon Inspector access to AWS Services needed to perform security assessments. You can create up to 50 assessment targets per AWS account. You can run up to 500 concurrent agents per AWS account. For more information, see <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_applications.html"> Amazon Inspector Assessment Targets</a>.</p>
    fn create_assessment_target(
        &self,
        input: CreateAssessmentTargetRequest,
    ) -> RusotoFuture<CreateAssessmentTargetResponse, CreateAssessmentTargetError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.CreateAssessmentTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAssessmentTargetResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateAssessmentTargetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Creates an assessment template for the assessment target that is specified by the ARN of the assessment target. If the <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_slr.html">service-linked role</a> isn’t already registered, this action also creates and registers a service-linked role to grant Amazon Inspector access to AWS Services needed to perform security assessments.</p>
    fn create_assessment_template(
        &self,
        input: CreateAssessmentTemplateRequest,
    ) -> RusotoFuture<CreateAssessmentTemplateResponse, CreateAssessmentTemplateError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.CreateAssessmentTemplate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateAssessmentTemplateResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateAssessmentTemplateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Starts the generation of an exclusions preview for the specified assessment template. The exclusions preview lists the potential exclusions (ExclusionPreview) that Inspector can detect before it runs the assessment. </p>
    fn create_exclusions_preview(
        &self,
        input: CreateExclusionsPreviewRequest,
    ) -> RusotoFuture<CreateExclusionsPreviewResponse, CreateExclusionsPreviewError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.CreateExclusionsPreview");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateExclusionsPreviewResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(CreateExclusionsPreviewError::from_response(response))
                }))
            }
        })
    }

    /// <p>Creates a resource group using the specified set of tags (key and value pairs) that are used to select the EC2 instances to be included in an Amazon Inspector assessment target. The created resource group is then used to create an Amazon Inspector assessment target. For more information, see <a>CreateAssessmentTarget</a>.</p>
    fn create_resource_group(
        &self,
        input: CreateResourceGroupRequest,
    ) -> RusotoFuture<CreateResourceGroupResponse, CreateResourceGroupError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.CreateResourceGroup");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<CreateResourceGroupResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(CreateResourceGroupError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the assessment run that is specified by the ARN of the assessment run.</p>
    fn delete_assessment_run(
        &self,
        input: DeleteAssessmentRunRequest,
    ) -> RusotoFuture<(), DeleteAssessmentRunError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DeleteAssessmentRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteAssessmentRunError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the assessment target that is specified by the ARN of the assessment target.</p>
    fn delete_assessment_target(
        &self,
        input: DeleteAssessmentTargetRequest,
    ) -> RusotoFuture<(), DeleteAssessmentTargetError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DeleteAssessmentTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DeleteAssessmentTargetError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Deletes the assessment template that is specified by the ARN of the assessment template.</p>
    fn delete_assessment_template(
        &self,
        input: DeleteAssessmentTemplateRequest,
    ) -> RusotoFuture<(), DeleteAssessmentTemplateError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DeleteAssessmentTemplate");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DeleteAssessmentTemplateError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn describe_assessment_runs(
        &self,
        input: DescribeAssessmentRunsRequest,
    ) -> RusotoFuture<DescribeAssessmentRunsResponse, DescribeAssessmentRunsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeAssessmentRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAssessmentRunsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeAssessmentRunsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes the assessment targets that are specified by the ARNs of the assessment targets.</p>
    fn describe_assessment_targets(
        &self,
        input: DescribeAssessmentTargetsRequest,
    ) -> RusotoFuture<DescribeAssessmentTargetsResponse, DescribeAssessmentTargetsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeAssessmentTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAssessmentTargetsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAssessmentTargetsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the assessment templates that are specified by the ARNs of the assessment templates.</p>
    fn describe_assessment_templates(
        &self,
        input: DescribeAssessmentTemplatesRequest,
    ) -> RusotoFuture<DescribeAssessmentTemplatesResponse, DescribeAssessmentTemplatesError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "InspectorService.DescribeAssessmentTemplates",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeAssessmentTemplatesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeAssessmentTemplatesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the IAM role that enables Amazon Inspector to access your AWS account.</p>
    fn describe_cross_account_access_role(
        &self,
    ) -> RusotoFuture<DescribeCrossAccountAccessRoleResponse, DescribeCrossAccountAccessRoleError>
    {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "InspectorService.DescribeCrossAccountAccessRole",
        );
        request.set_payload(Some(bytes::Bytes::from_static(b"{}")));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeCrossAccountAccessRoleResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(DescribeCrossAccountAccessRoleError::from_response(response))
                }))
            }
        })
    }

    /// <p>Describes the exclusions that are specified by the exclusions' ARNs.</p>
    fn describe_exclusions(
        &self,
        input: DescribeExclusionsRequest,
    ) -> RusotoFuture<DescribeExclusionsResponse, DescribeExclusionsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeExclusions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeExclusionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeExclusionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the findings that are specified by the ARNs of the findings.</p>
    fn describe_findings(
        &self,
        input: DescribeFindingsRequest,
    ) -> RusotoFuture<DescribeFindingsResponse, DescribeFindingsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeFindings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeFindingsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(DescribeFindingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Describes the resource groups that are specified by the ARNs of the resource groups.</p>
    fn describe_resource_groups(
        &self,
        input: DescribeResourceGroupsRequest,
    ) -> RusotoFuture<DescribeResourceGroupsResponse, DescribeResourceGroupsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeResourceGroups");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeResourceGroupsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeResourceGroupsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Describes the rules packages that are specified by the ARNs of the rules packages.</p>
    fn describe_rules_packages(
        &self,
        input: DescribeRulesPackagesRequest,
    ) -> RusotoFuture<DescribeRulesPackagesResponse, DescribeRulesPackagesError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.DescribeRulesPackages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<DescribeRulesPackagesResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(DescribeRulesPackagesError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Produces an assessment report that includes detailed and comprehensive results of a specified assessment run. </p>
    fn get_assessment_report(
        &self,
        input: GetAssessmentReportRequest,
    ) -> RusotoFuture<GetAssessmentReportResponse, GetAssessmentReportError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.GetAssessmentReport");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetAssessmentReportResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetAssessmentReportError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Retrieves the exclusions preview (a list of ExclusionPreview objects) specified by the preview token. You can obtain the preview token by running the CreateExclusionsPreview API.</p>
    fn get_exclusions_preview(
        &self,
        input: GetExclusionsPreviewRequest,
    ) -> RusotoFuture<GetExclusionsPreviewResponse, GetExclusionsPreviewError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.GetExclusionsPreview");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetExclusionsPreviewResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetExclusionsPreviewError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Information about the data that is collected for the specified assessment run.</p>
    fn get_telemetry_metadata(
        &self,
        input: GetTelemetryMetadataRequest,
    ) -> RusotoFuture<GetTelemetryMetadataResponse, GetTelemetryMetadataError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.GetTelemetryMetadata");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<GetTelemetryMetadataResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(GetTelemetryMetadataError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the agents of the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn list_assessment_run_agents(
        &self,
        input: ListAssessmentRunAgentsRequest,
    ) -> RusotoFuture<ListAssessmentRunAgentsResponse, ListAssessmentRunAgentsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListAssessmentRunAgents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAssessmentRunAgentsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAssessmentRunAgentsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists the assessment runs that correspond to the assessment templates that are specified by the ARNs of the assessment templates.</p>
    fn list_assessment_runs(
        &self,
        input: ListAssessmentRunsRequest,
    ) -> RusotoFuture<ListAssessmentRunsResponse, ListAssessmentRunsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListAssessmentRuns");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAssessmentRunsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListAssessmentRunsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists the ARNs of the assessment targets within this AWS account. For more information about assessment targets, see <a href="https://docs.aws.amazon.com/inspector/latest/userguide/inspector_applications.html">Amazon Inspector Assessment Targets</a>.</p>
    fn list_assessment_targets(
        &self,
        input: ListAssessmentTargetsRequest,
    ) -> RusotoFuture<ListAssessmentTargetsResponse, ListAssessmentTargetsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListAssessmentTargets");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAssessmentTargetsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListAssessmentTargetsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Lists the assessment templates that correspond to the assessment targets that are specified by the ARNs of the assessment targets.</p>
    fn list_assessment_templates(
        &self,
        input: ListAssessmentTemplatesRequest,
    ) -> RusotoFuture<ListAssessmentTemplatesResponse, ListAssessmentTemplatesError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListAssessmentTemplates");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListAssessmentTemplatesResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(ListAssessmentTemplatesError::from_response(response))
                }))
            }
        })
    }

    /// <p>Lists all the event subscriptions for the assessment template that is specified by the ARN of the assessment template. For more information, see <a>SubscribeToEvent</a> and <a>UnsubscribeFromEvent</a>.</p>
    fn list_event_subscriptions(
        &self,
        input: ListEventSubscriptionsRequest,
    ) -> RusotoFuture<ListEventSubscriptionsResponse, ListEventSubscriptionsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListEventSubscriptions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListEventSubscriptionsResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListEventSubscriptionsError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>List exclusions that are generated by the assessment run.</p>
    fn list_exclusions(
        &self,
        input: ListExclusionsRequest,
    ) -> RusotoFuture<ListExclusionsResponse, ListExclusionsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListExclusions");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListExclusionsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListExclusionsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists findings that are generated by the assessment runs that are specified by the ARNs of the assessment runs.</p>
    fn list_findings(
        &self,
        input: ListFindingsRequest,
    ) -> RusotoFuture<ListFindingsResponse, ListFindingsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListFindings");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListFindingsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListFindingsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all available Amazon Inspector rules packages.</p>
    fn list_rules_packages(
        &self,
        input: ListRulesPackagesRequest,
    ) -> RusotoFuture<ListRulesPackagesResponse, ListRulesPackagesError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListRulesPackages");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListRulesPackagesResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(ListRulesPackagesError::from_response(response))),
                )
            }
        })
    }

    /// <p>Lists all tags associated with an assessment template.</p>
    fn list_tags_for_resource(
        &self,
        input: ListTagsForResourceRequest,
    ) -> RusotoFuture<ListTagsForResourceResponse, ListTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.ListTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<ListTagsForResourceResponse, _>()
                }))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(ListTagsForResourceError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Previews the agents installed on the EC2 instances that are part of the specified assessment target.</p>
    fn preview_agents(
        &self,
        input: PreviewAgentsRequest,
    ) -> RusotoFuture<PreviewAgentsResponse, PreviewAgentsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.PreviewAgents");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<PreviewAgentsResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(PreviewAgentsError::from_response(response))),
                )
            }
        })
    }

    /// <p>Registers the IAM role that grants Amazon Inspector access to AWS Services needed to perform security assessments.</p>
    fn register_cross_account_access_role(
        &self,
        input: RegisterCrossAccountAccessRoleRequest,
    ) -> RusotoFuture<(), RegisterCrossAccountAccessRoleError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "InspectorService.RegisterCrossAccountAccessRole",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RegisterCrossAccountAccessRoleError::from_response(response))
                }))
            }
        })
    }

    /// <p>Removes entire attributes (key and value pairs) from the findings that are specified by the ARNs of the findings where an attribute with the specified key exists.</p>
    fn remove_attributes_from_findings(
        &self,
        input: RemoveAttributesFromFindingsRequest,
    ) -> RusotoFuture<RemoveAttributesFromFindingsResponse, RemoveAttributesFromFindingsError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header(
            "x-amz-target",
            "InspectorService.RemoveAttributesFromFindings",
        );
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<RemoveAttributesFromFindingsResponse, _>()
                }))
            } else {
                Box::new(response.buffer().from_err().and_then(|response| {
                    Err(RemoveAttributesFromFindingsError::from_response(response))
                }))
            }
        })
    }

    /// <p>Sets tags (key and value pairs) to the assessment template that is specified by the ARN of the assessment template.</p>
    fn set_tags_for_resource(
        &self,
        input: SetTagsForResourceRequest,
    ) -> RusotoFuture<(), SetTagsForResourceError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.SetTagsForResource");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SetTagsForResourceError::from_response(response))),
                )
            }
        })
    }

    /// <p>Starts the assessment run specified by the ARN of the assessment template. For this API to function properly, you must not exceed the limit of running up to 500 concurrent agents per AWS account.</p>
    fn start_assessment_run(
        &self,
        input: StartAssessmentRunRequest,
    ) -> RusotoFuture<StartAssessmentRunResponse, StartAssessmentRunError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.StartAssessmentRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(response.buffer().from_err().and_then(|response| {
                    proto::json::ResponsePayload::new(&response)
                        .deserialize::<StartAssessmentRunResponse, _>()
                }))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StartAssessmentRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Stops the assessment run that is specified by the ARN of the assessment run.</p>
    fn stop_assessment_run(
        &self,
        input: StopAssessmentRunRequest,
    ) -> RusotoFuture<(), StopAssessmentRunError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.StopAssessmentRun");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(StopAssessmentRunError::from_response(response))),
                )
            }
        })
    }

    /// <p>Enables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic.</p>
    fn subscribe_to_event(
        &self,
        input: SubscribeToEventRequest,
    ) -> RusotoFuture<(), SubscribeToEventError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.SubscribeToEvent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response
                        .buffer()
                        .from_err()
                        .and_then(|response| Err(SubscribeToEventError::from_response(response))),
                )
            }
        })
    }

    /// <p>Disables the process of sending Amazon Simple Notification Service (SNS) notifications about a specified event to a specified SNS topic.</p>
    fn unsubscribe_from_event(
        &self,
        input: UnsubscribeFromEventRequest,
    ) -> RusotoFuture<(), UnsubscribeFromEventError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.UnsubscribeFromEvent");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UnsubscribeFromEventError::from_response(response))
                    }),
                )
            }
        })
    }

    /// <p>Updates the assessment target that is specified by the ARN of the assessment target.</p> <p>If resourceGroupArn is not specified, all EC2 instances in the current AWS account and region are included in the assessment target.</p>
    fn update_assessment_target(
        &self,
        input: UpdateAssessmentTargetRequest,
    ) -> RusotoFuture<(), UpdateAssessmentTargetError> {
        let mut request = SignedRequest::new("POST", "inspector", &self.region, "/");

        request.set_content_type("application/x-amz-json-1.1".to_owned());
        request.add_header("x-amz-target", "InspectorService.UpdateAssessmentTarget");
        let encoded = serde_json::to_string(&input).unwrap();
        request.set_payload(Some(encoded));

        self.client.sign_and_dispatch(request, |response| {
            if response.status.is_success() {
                Box::new(future::ok(::std::mem::drop(response)))
            } else {
                Box::new(
                    response.buffer().from_err().and_then(|response| {
                        Err(UpdateAssessmentTargetError::from_response(response))
                    }),
                )
            }
        })
    }
}
