# 🪣 S3 Chelak

## Description

**S3 Chelak** is a local development tool that emulates the AWS S3 API's interface. This allows you to test your S3 integrations without connecting to AWS, which is useful for offline development and testing. It's designed to behave as much as possible like AWS S3, with the ability to store and retrieve objects, among other functions.

## Features

- Emulates the S3 ListBuckets (so far)
- Error handling similar to the AWS S3 API.

## Installation

```bash
$ git clone https://github.com/oneslash/s3-chelak.git
$ cd s3-chelak
$ cargo build
```

## Usage

Start the server:

```bash
$ cargo run
```

Parameters:

- `server_url` (Default: `"localhost"`): Defines the URL at which the server will be running. You can modify this value according to your needs.

- `server_port` (Default: `"9090"`): This is the port number on which the server will listen for requests. If you have another service running on the default port, you may want to change this.
- `working_folder` (Default: `"/tmp/s3-server"`): This is the directory where the server will store all the data. If you wish to have the data stored in a different location, you can modify this value.

```bash
$ ./s3-chelak --server-url "my_custom_url" --server-port "8080" --working-folder "/path/to/my/folder"
```

## API's implemented

| AWS S3 API Name                             |    Implemented     |
| ------------------------------------------- | :----------------: |
| AbortMultipartUpload                        |        :x:         |
| CompleteMultipartUpload                     |        :x:         |
| CopyObject                                  |        :x:         |
| CreateBucket                                | :white_check_mark: |
| CreateMultipartUpload                       |        :x:         |
| DeleteBucket                                |        :x:         |
| DeleteBucketAnalyticsConfiguration          |        :x:         |
| DeleteBucketCors                            |        :x:         |
| DeleteBucketEncryption                      |        :x:         |
| DeleteBucketIntelligentTieringConfiguration |        :x:         |
| DeleteBucketInventoryConfiguration          |        :x:         |
| DeleteBucketLifecycle                       |        :x:         |
| DeleteBucketMetricsConfiguration            |        :x:         |
| DeleteBucketOwnershipControls               |        :x:         |
| DeleteBucketPolicy                          |        :x:         |
| DeleteBucketReplication                     |        :x:         |
| DeleteBucketTagging                         |        :x:         |
| DeleteBucketWebsite                         |        :x:         |
| DeleteObject                                |        :x:         |
| DeleteObjectTagging                         |        :x:         |
| DeleteObjects                               |        :x:         |
| DeletePublicAccessBlock                     |        :x:         |
| GetBucketAccelerateConfiguration            |        :x:         |
| GetBucketAcl                                |        :x:         |
| GetBucketAnalyticsConfiguration             |        :x:         |
| GetBucketCors                               |        :x:         |
| GetBucketEncryption                         |        :x:         |
| GetBucketIntelligentTieringConfiguration    |        :x:         |
| GetBucketInventoryConfiguration             |        :x:         |
| GetBucketLifecycle                          |        :x:         |
| GetBucketLifecycleConfiguration             |        :x:         |
| GetBucketLocation                           |        :x:         |
| GetBucketLogging                            |        :x:         |
| GetBucketMetricsConfiguration               |        :x:         |
| GetBucketNotification                       |        :x:         |
| GetBucketNotificationConfiguration          |        :x:         |
| GetBucketOwnershipControls                  |        :x:         |
| GetBucketPolicy                             |        :x:         |
| GetBucketPolicyStatus                       |        :x:         |
| GetBucketReplication                        |        :x:         |
| GetBucketRequestPayment                     |        :x:         |
| GetBucketTagging                            |        :x:         |
| GetBucketVersioning                         |        :x:         |
| GetBucketWebsite                            |        :x:         |
| GetObject                                   |        :x:         |
| GetObjectAcl                                |        :x:         |
| GetObjectLegalHold                          |        :x:         |
| GetObjectLockConfiguration                  |        :x:         |
| GetObjectRetention                          |        :x:         |
| GetObjectTagging                            |        :x:         |
| GetObjectTorrent                            |        :x:         |
| GetPublicAccessBlock                        |        :x:         |
| HeadBucket                                  |        :x:         |
| HeadObject                                  |        :x:         |
| ListBucketAnalyticsConfigurations           |        :x:         |
| ListBucketIntelligentTieringConfigurations  |        :x:         |
| ListBucketInventoryConfigurations           |        :x:         |
| ListBucketMetricsConfigurations             |        :x:         |
| ListBuckets                                 | :white_check_mark: |
| ListMultipartUploads                        |        :x:         |
| ListObjectVersions                          |        :x:         |
| ListObjects                                 |        :x:         |
| ListObjectsV2                               |        :x:         |
| ListParts                                   |        :x:         |
| PutBucketAccelerateConfiguration            |        :x:         |
| PutBucketAcl                                |        :x:         |
| PutBucketAnalyticsConfiguration             |        :x:         |
| PutBucketCors                               |        :x:         |
| PutBucketEncryption                         |        :x:         |
| PutBucketIntelligentTieringConfiguration    |        :x:         |
| PutBucketInventoryConfiguration             |        :x:         |
| PutBucketLifecycle                          |        :x:         |
| PutBucketLifecycleConfiguration             |        :x:         |
| PutBucketLifecycleConfiguration             |        :x:         |
| PutBucketLogging                            |        :x:         |
| PutBucketMetricsConfiguration               |        :x:         |
| PutBucketNotification                       |        :x:         |
| PutBucketNotificationConfiguration          |        :x:         |
| PutBucketOwnershipControls                  |        :x:         |
| PutBucketPolicy                             |        :x:         |
| PutBucketReplication                        |        :x:         |
| PutBucketRequestPayment                     |        :x:         |
| PutBucketTagging                            |        :x:         |
| PutBucketVersioning                         |        :x:         |
| PutBucketWebsite                            |        :x:         |
| PutObject                                   |        :x:         |
| PutObjectAcl                                |        :x:         |
| PutObjectLegalHold                          |        :x:         |
| PutObjectLockConfiguration                  |        :x:         |
| PutObjectRetention                          |        :x:         |
| PutObjectTagging                            |        :x:         |
| PutPublicAccessBlock                        |        :x:         |
| RestoreObject                               |        :x:         |
| SelectObjectContent                         |        :x:         |
| UploadPart                                  |        :x:         |
| UploadPartCopy                              |        :x:         |
| WriteGetObjectResponse                      |        :x:         |

## License

This project is licensed under the MIT License. See the [LICENSE.md](https://chat.openai.com/LICENSE.md) file for details.

## Acknowledgments

- AWS for their comprehensive and well-documented S3 API.