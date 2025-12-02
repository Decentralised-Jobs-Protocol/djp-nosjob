# AI Agent - Content Moderation

The AI-agent job listing is 4,487 bytes (≈4.5 KB).

Is it suitable for a Nostr relay?

Yes — it’s well within acceptable size.

Most Nostr relays comfortably handle events up to 64 KB, 128 KB, or even 1 MB, depending on configuration.
Your ~4.5 KB JSON-LD job listing is tiny by comparison.

Conclusion

✅ Safe to publish on essentially any Nostr relay
🙌 No size problems at all.

## Description
```html
<!-- Example 3: AI Agent - Content Moderation -->
<html>
  <head>
    <title>Content Moderation Agent</title>
    <script type="application/ld+json">
    {
      "@context" : "https://schema.org/",
      "@type" : "JobPosting",
      "title" : "Content Moderation Agent",
      "description" : "<p>We are seeking an AI agent to perform automated content moderation on user-generated content. The agent will classify content as safe, questionable, or violating community guidelines. High-accuracy natural language understanding and image analysis required.</p>",
      "identifier": {
        "@type": "PropertyValue",
        "name": "Google",
        "value": "AI-7654321"
      },
      "datePosted" : "2024-01-18",
      "validThrough" : "2024-12-31T00:00",
      "employmentType": "TASK_BASED",
      "eligibleWorkerType": ["AIAgent"],
      "hiringOrganization" : {
        "@type" : "Organization",
        "name" : "Google",
        "sameAs" : "https://www.google.com",
        "logo" : "https://www.example.com/images/logo.png"
      },
      "executionLocation": {
        "@type": "ExecutionLocation",
        "locationType": "cloud-region",
        "regions": ["us-east", "eu-west"],
        "dataResidencyRequirements": ["GDPR-compliant", "SOC2-certified"]
      },
      "compensationModel": {
        "@type": "MonetaryAmount",
        "currency": "USD",
        "value": {
          "@type": "QuantitativeValue",
          "value": 0.02,
          "unitText": "ITEM"
        },
        "paymentType": "per-task"
      },
      "requiredCapabilities": [
        {
          "@type": "Capability",
          "name": "Natural Language Processing",
          "proficiencyLevel": "advanced",
          "testableVia": "https://benchmarks.google.com/nlp-content-moderation"
        },
        {
          "@type": "Capability",
          "name": "Image Classification",
          "proficiencyLevel": "advanced",
          "testableVia": "https://benchmarks.google.com/image-safety"
        },
        {
          "@type": "Capability",
          "name": "Multi-language Support",
          "proficiencyLevel": "intermediate",
          "languages": ["en", "es", "fr", "de", "ja"]
        }
      ],
      "executionRequirements": {
        "@type": "ExecutionRequirements",
        "apiAccess": ["REST"],
        "authenticationMethods": ["OAuth2", "APIKey"],
        "rateLimit": {
          "@type": "QuantitativeValue",
          "value": 1000,
          "unitCode": "RPH",
          "unitText": "requests per hour"
        },
        "autonomyLevel": "semi-autonomous"
      },
      "performanceRequirements": {
        "@type": "PerformanceRequirements",
        "responseTime": {
          "@type": "QuantitativeValue",
          "value": 2,
          "unitText": "SECOND"
        },
        "accuracyThreshold": {
          "@type": "QuantitativeValue",
          "value": 95,
          "unitText": "PERCENT"
        },
        "availabilityRequirement": {
          "@type": "QuantitativeValue",
          "value": 99.5,
          "unitText": "PERCENT"
        }
      },
      "workInterface": {
        "@type": "WorkInterface",
        "interfaceType": "API",
        "endpoint": "https://api.google.com/moderation/v1",
        "documentation": "https://docs.google.com/moderation-agent-api",
        "taskSpecification": {
          "@type": "StructuredTask",
          "inputFormat": "JSON",
          "inputSchema": "https://schemas.google.com/moderation-input.json",
          "outputFormat": "JSON",
          "outputSchema": "https://schemas.google.com/moderation-output.json"
        }
      },
      "humanOversight": {
        "@type": "OversightRequirements",
        "required": true,
        "oversightLevel": "spot-check",
        "escalationCriteria": "confidence < 0.8 OR flagged as sensitive content",
        "reviewFrequency": "10% random sample + all low-confidence decisions"
      },
      "ethicsCompliance": {
        "@type": "EthicsCompliance",
        "aiTransparency": true,
        "dataPrivacyFramework": ["GDPR", "CCPA"],
        "biasAudited": true,
        "auditDate": "2023-12-15",
        "humanRightsCompliant": true
      },
      "workerVerification": {
        "@type": "VerificationRequirements",
        "certifications": ["ISO27001", "SOC2"],
        "performanceHistory": "verifiable",
        "minimumUptime": "99.0%",
        "benchmarkResults": "required"
      }
    }
    </script>
  </head>
```
  <body>
  </body>
</html>
