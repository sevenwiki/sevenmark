# Real-World Usage Examples

<div v-pre>

This section demonstrates practical SevenMark usage for common documentation scenarios.

## Software Project README

```sevenmark
# MyAwesome Project

{{{ #style="background: #d4edda; border: 1px solid #c3e6cb; padding: 10px; border-radius: 5px"
🚀 **A powerful web application built with modern technologies**
}}}

## Quick Start!

{{{#list #1
[[Clone the repository: {{{#code git clone https://github.com/user/project.git }}}]]
[[Install dependencies: {{{#code npm install }}}]]
[[Set up environment: {{{#code cp .env.example .env }}}]]
[[Start development server: {{{#code npm run dev }}}]]
}}}

## Features

{{{#table
[[[[Feature]] [[Status]] [[Version]]]]
[[[[🔐 Authentication]] [[✅ Complete]] [[v1.0]]]]
[[[[📊 Dashboard]] [[🚧 In Progress]] [[v1.1]]]]
[[[[🔔 Notifications]] [[📋 Planned]] [[v1.2]]]]
}}}

## API Documentation!

{{{#fold
[[📖 **API Endpoints** - Click to view]]
[[
### User Management

{{{#code #lang="http"
GET    /api/users
POST   /api/users
PUT    /api/users/:id
DELETE /api/users/:id
}}}

### Authentication

{{{#code #lang="javascript"
// Login example
const response = await fetch('/api/auth/login', {
  method: 'POST',
  headers: { 'Content-Type': 'application/json' },
  body: JSON.stringify({ email, password })
});
}}}
]]
}}}

## Contributing

Please read our [[#url="https://example.com/CONTRIBUTING.md" contribution guidelines]] before submitting PRs.

{{{#quote
"Code is like humor. When you have to explain it, it's bad." - Cory House
}}}

---

**License:** MIT | **Last Updated:** [now]
```

## Technical Documentation

```sevenmark
# System Architecture Guide

## Overview

Our system follows a **microservices architecture** with the following components:

{{{#list #1 #style="background: #f8f9fa; padding: 15px; border-radius: 5px"
[[**Frontend Layer**
   - React.js SPA
   - Redux state management  
   - Responsive design]]
[[**API Gateway**
   - Request routing
   - Authentication middleware
   - Rate limiting]]
[[**Backend Services**
   {{{#list #a
   [[User Service (Node.js)]]
   [[Payment Service (Python)]]
   [[Notification Service (Go)]]
   }}}]]
[[**Data Layer**
   {{{#list #a
   [[PostgreSQL (primary data)]]
   [[Redis (caching)]]
   [[Elasticsearch (search)]]
   }}}]]
}}}

## Service Communication!

{{{#fold #style="border: 2px solid #007bff; background: #e7f1ff"
[[⚙️ **Communication Patterns** - Technical Details]]
[[
### Synchronous Communication

{{{#table
[[[[Service A]] [[Service B]] [[Protocol]] [[Use Case]]]]
[[[[API Gateway]] [[User Service]] [[HTTP/REST]] [[User authentication]]]]
[[[[Frontend]] [[API Gateway]] [[HTTP/HTTPS]] [[All client requests]]]]
}}}

### Asynchronous Communication

{{{#code #lang="yaml"
# Message Queue Configuration
queues:
  user_events:
    exchange: "users"
    routing_key: "user.created"
    consumers:
      - notification_service
      - analytics_service
  
  payment_events:
    exchange: "payments"
    routing_key: "payment.completed"
    consumers:
      - user_service
      - email_service
}}}
]]
}}}

## Deployment Architecture

{{{#tex #block
\text{Load Balancer} \rightarrow \text{API Gateway} \rightarrow \text{Microservices}
}}}

### Infrastructure Components

- **Container Orchestration:** Kubernetes
- **Service Mesh:** Istio for traffic management
- **Monitoring:** Prometheus + Grafana
- **Logging:** ELK Stack (Elasticsearch, Logstash, Kibana)

/* Architecture review notes:
   - Consider adding circuit breaker pattern
   - Evaluate need for event sourcing
   - Plan for horizontal scaling
*/
```

## Meeting Minutes Template

```sevenmark
# Team Meeting - Product Planning

**Date:** [now]  
**Attendees:** Alice, Bob, Charlie, Diana

## Agenda!

{{{#list #1
[[Product roadmap review]]
[[Q4 sprint planning]]
[[Technical debt discussion]]
[[Action items from last meeting]]
}}}

## Discussion Points

### Q4 Roadmap Status!

{{{#table #style="margin: 10px 0"
[[[[Feature]] [[Owner]] [[Status]] [[Target Date]]]]
[[[[User Dashboard]] [[Alice]] [[🟢 On Track]] [[Oct 15]]]]
[[[[Mobile App]] [[Bob]] [[🟡 At Risk]] [[Nov 30]]]]
[[[[API v2]] [[Charlie]] [[🔴 Delayed]] [[Dec 15]]]]
}}}

### Technical Debt Items

{{{#list #A #style="background: #fff3cd; padding: 10px; border-radius: 3px"
[[**Database optimization** - Query performance issues
   - Impact: High ⚠️
   - Effort: 3 weeks
   - Owner: Diana]]
[[**Legacy code refactoring** - Authentication module
   - Impact: Medium
   - Effort: 2 weeks  
   - Owner: Charlie]]
[[**Test coverage improvement** - Current: 60%, Target: 80%
   - Impact: Medium
   - Effort: Ongoing
   - Owner: Team]]
}}}

## Decisions Made

{{{#quote #style="background: #d1ecf1; border-left: 4px solid #bee5eb; padding: 10px"
**DECISION:** Postpone API v2 release to ensure quality. 
Focus team resources on mobile app delivery.
**Rationale:** Customer feedback prioritizes mobile experience.
}}}

## Action Items

{{{#list #1
[[📋 Alice: Update project timeline by Friday]]
[[🔧 Bob: Investigate mobile app performance issues]]
[[📊 Charlie: Provide revised API v2 estimate]]
[[📈 Diana: Begin database optimization analysis]]
}}}

---

**Next Meeting:** [age(2024-01-07)] // Next Friday at 2 PM
```

## User Manual Example  

```sevenmark
{{{#category User Documentation}}}

# Getting Started with DataAnalyzer Pro

{{{#include #page="LegalDisclaimer" Standard disclaimer text}}}

## Installation Guide!

{{{#fold
[[💿 **Installation Steps** - Choose your platform]]
[[
### Windows Installation

{{{#list #1
[[Download installer from [[#url="https://download.example.com/windows" official website]]]]
[[Run **DataAnalyzer-Setup.exe** as administrator]]
[[Follow the setup wizard]]
[[Launch application from Start Menu]]
}}}

### macOS Installation  

{{{#code #lang="bash"
# Using Homebrew
brew install --cask dataanalyzer-pro

# Or download DMG from website
curl -O https://download.example.com/mac/DataAnalyzer.dmg
}}}

### Linux Installation

{{{#code #lang="bash"
# Ubuntu/Debian
sudo apt install dataanalyzer-pro

# Red Hat/CentOS  
sudo yum install dataanalyzer-pro

# From source
git clone https://github.com/company/dataanalyzer.git
cd dataanalyzer && make install
}}}
]]
}}}

## Basic Usage

### Creating Your First Project

{{{#list #1 #style="counter-reset: step-counter"
[[Open DataAnalyzer Pro]]
[[Click **"New Project"** or press Ctrl+N]]  
[[Choose data source:
   {{{#list #a
   [[CSV file import]]
   [[Database connection]]
   [[API endpoint]]
   }}}]]
[[Configure data schema and types]]
[[Begin analysis with built-in tools]]
}}}

### Common Tasks

{{{#table
[[[[Task]] [[Menu Path]] [[Keyboard Shortcut]] [[Description]]]]
[[[[Import Data]] [[File → Import]] [[Ctrl+I]] [[Load data from various sources]]]]
[[[[Create Chart]] [[Insert → Chart]] [[Ctrl+Shift+C]] [[Generate visualizations]]]]
[[[[Export Results]] [[File → Export]] [[Ctrl+E]] [[Save analysis results]]]]
[[[[Run Analysis]] [[Analysis → Execute]] [[F5]] [[Execute current analysis]]]]
}}}

## Troubleshooting!

{{{#fold #style="background: #f8d7da; border: 1px solid #f5c6cb"
[[⚠️ **Common Issues** - Solutions and fixes]]
[[
### Memory Issues

**Problem:** Application crashes with large datasets

**Solution:**
{{{#list #1
[[Increase JVM heap size: {{{#code -Xmx8g }}}]]
[[Enable data streaming mode in preferences]]
[[Split large files into smaller chunks]]
}}}

### Connection Problems

**Problem:** Cannot connect to database

**Solution:**
{{{#code #lang="sql"
-- Test connection manually
SELECT 1 FROM dual;

-- Check firewall settings  
telnet database-server 5432
}}}

### Performance Issues

**Formula for optimal performance:**
{{{#tex #block
\text{Performance} = \frac{\text{Available RAM}}{\text{Dataset Size}} \times \text{CPU Cores}
}}}
]]
}}}

## Support

Need help? Contact our support team:

- 📧 Contact: [[#url="https://support.example.com/contact" support@example.com]]
- 💬 Live Chat: Available 9 AM - 5 PM EST  
- 📚 Knowledge Base: [[#url="https://help.example.com" help.example.com]]

{{{#quote #style="text-align: center; font-style: italic"
"DataAnalyzer Pro - Transform your data into insights"
}}}

---
*Version 3.2.1 | Updated [now] | © 2024 Example Corp*
```

</div>
