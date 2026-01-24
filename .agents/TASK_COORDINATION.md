# Task Coordination Guide

**Purpose**: How agents coordinate and work together on tasks

---

## 🔄 Coordination Patterns

### Pattern 1: Sequential (One After Another)
```
Rust Agent → UI/UX Agent → Browser Agent → Code Review Agent
```
**Example**: New feature implementation
1. Rust Agent implements logic
2. UI/UX Agent styles it
3. Browser Agent reviews visually
4. Code Review Agent reviews code

### Pattern 2: Parallel (At Same Time)
```
Rust Agent + UI/UX Agent (different parts) → Browser Agent
```
**Example**: Feature with separate logic and UI
1. Rust Agent and UI/UX Agent work simultaneously
2. Browser Agent reviews when both complete

### Pattern 3: Review Cycle
```
Implementation → Browser Agent → Code Review Agent → Supervisor → Approval
```
**Example**: Any change
1. Agent implements
2. Browser Agent reviews visually
3. Code Review Agent reviews code
4. Supervisor approves

---

## 📋 Task Assignment Matrix

| Task | Primary Agent | Supporting Agents | Review Agents |
|------|--------------|------------------|---------------|
| Fix compilation error | Rust Agent | - | Code Review Agent |
| Implement priority system | Rust Agent | UI/UX Agent | Browser Agent, Code Review Agent |
| Style priority UI | UI/UX Agent | - | Browser Agent |
| Implement Today filter | Rust Agent | - | Code Review Agent |
| Implement Upcoming filter | Rust Agent | - | Code Review Agent |
| Bottom tab navigation | Rust Agent | UI/UX Agent | Browser Agent |
| Settings screen | Rust Agent + UI/UX Agent | - | Browser Agent, Code Review Agent |
| Fix Lighthouse issues | UI/UX Agent | Browser Agent | Supervisor |
| Visual polish | UI/UX Agent | - | Browser Agent |
| Performance optimization | Rust Agent | Browser Agent | Code Review Agent |
| Create tests | Testing Agent | Rust Agent | Code Review Agent |
| Update documentation | Documentation Agent | - | Supervisor |

---

## 🤝 Agent Handoffs

### Rust Agent → UI/UX Agent
**When**: After implementing new feature that needs styling
**Handoff**: "Feature X implemented, ready for styling"
**UI/UX Agent**: Styles the feature, reports back

### UI/UX Agent → Browser Agent
**When**: After styling changes
**Handoff**: "Styling complete, ready for visual review"
**Browser Agent**: Reviews, takes screenshots, reports

### Any Agent → Code Review Agent
**When**: After code changes
**Handoff**: "Code changes complete, ready for review"
**Code Review Agent**: Reviews, provides feedback

### All Agents → Supervisor
**When**: Task complete or blocker encountered
**Handoff**: Daily standup report
**Supervisor**: Coordinates, approves, assigns next task

---

## 🚨 Escalation Path

```
Agent encounters blocker
    ↓
Report to Supervisor immediately
    ↓
Supervisor assesses
    ↓
If technical: Assign to appropriate agent
If priority: Adjust plan
If critical: All agents focus on fix
```

---

## 📊 Dependency Tracking

### Day 1 Dependencies
- Rust Agent (filters) → Browser Agent (test filters)
- UI/UX Agent (layout) → Browser Agent (visual review)

### Day 2 Dependencies
- Rust Agent (priority) → UI/UX Agent (style priority)
- Rust Agent (navigation) → UI/UX Agent (style navigation)
- Both → Browser Agent (test everything)

### Day 3 Dependencies
- Rust Agent (settings) → UI/UX Agent (style settings)
- Both → Browser Agent (review settings)

### Day 4 Dependencies
- All Agents → Testing Agent (test everything)
- Testing Agent → Rust Agent (fix bugs found)

### Day 5 Dependencies
- Browser Agent (Lighthouse) → UI/UX Agent (fix issues)
- Browser Agent (performance) → Rust Agent (optimize)

### Day 6 Dependencies
- All Agents → Documentation Agent (document features)

### Day 7 Dependencies
- All Agents → Supervisor (final approval)

---

## 💬 Communication Templates

### Task Request (Agent to Supervisor)
```
Subject: Task Request - [Task Name]

Agent: [Agent Name]
Task: [Description]
Estimated Time: [Hours]
Dependencies: [List any]
Blockers: [None / Description]

Ready to start: [Yes/No]
```

### Task Completion (Agent to Supervisor)
```
Subject: Task Complete - [Task Name]

Agent: [Agent Name]
Task: [Description]
Status: ✅ Complete
Time Spent: [Hours]
Changes Made: [List files]
Next Steps: [What's needed next]
Ready for: [Which agent should review]
```

### Blocker Report (Agent to Supervisor)
```
Subject: 🚨 BLOCKER - [Issue Description]

Agent: [Agent Name]
Task: [Task Name]
Blocker: [Description]
Impact: [High/Medium/Low]
Help Needed From: [Agent Name]
Estimated Delay: [Hours/Days]
```

### Review Request (Agent to Review Agent)
```
Subject: Review Request - [Feature Name]

From: [Agent Name]
To: [Review Agent Name]
Feature: [Description]
Files Changed: [List]
Ready for Review: [Yes]
Priority: [High/Medium/Low]
```

---

## ✅ Quality Gates

Before any task is considered complete:

1. **Implementation**: Code works, no errors
2. **Visual Review**: Browser Agent approves (if UI change)
3. **Code Review**: Code Review Agent approves
4. **Supervisor Approval**: Final sign-off

---

## 📅 Daily Coordination Schedule

### Morning (9:00 AM)
- Agents start assigned tasks
- Report any blockers immediately

### Midday (12:00 PM)
- Quick status check (if needed)
- Coordinate handoffs

### Afternoon (3:00 PM)
- Continue work
- Prepare for standup

### End of Day (5:00 PM)
- Daily standup (15 min)
- Supervisor reviews progress
- Assign next day tasks

---

## 🎯 Success Indicators

### Good Coordination
- ✅ Tasks flow smoothly between agents
- ✅ No waiting on dependencies
- ✅ Clear communication
- ✅ Quick blocker resolution

### Poor Coordination (Red Flags)
- ❌ Agents waiting on each other
- ❌ Unclear handoffs
- ❌ Blockers not reported
- ❌ Duplicate work

---

**Last Updated**: 2026-01-15
