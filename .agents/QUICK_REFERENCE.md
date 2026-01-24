# Quick Reference - Multi-Agent System

**For**: Quick lookup of agent roles, workflows, and contacts

---

## 👥 Agent Directory

| Agent | Role | Primary Focus | Contact |
|-------|------|--------------|---------|
| **Supervisor** | Coordinator | Quality, approval, coordination | Always start here |
| **Rust Agent** | Backend | Rust code, logic, features | For code implementation |
| **UI/UX Agent** | Frontend | Styling, CSS, layout | For visual design |
| **Browser Agent** | Visual QA | Browser testing, Lighthouse | For visual review |
| **Code Review Agent** | Quality | Code quality, security | For code review |
| **Testing Agent** | QA | Tests, coverage | For testing |
| **Documentation Agent** | Docs | Documentation, changelog | For documentation |

---

## 🚀 Quick Workflows

### I need to implement a new feature
```
You → Supervisor
  → Supervisor delegates to Rust Agent (logic) + UI/UX Agent (styling)
  → Browser Agent reviews
  → Code Review Agent reviews
  → Supervisor approves
```

### I need to fix a visual issue
```
You → Supervisor
  → Supervisor delegates to UI/UX Agent
  → Browser Agent reviews
  → Supervisor approves
```

### I need to fix a bug
```
You → Supervisor
  → Supervisor delegates to Rust Agent
  → Testing Agent verifies
  → Browser Agent tests
  → Supervisor approves
```

### I need visual review
```
You → Supervisor
  → Supervisor delegates to Browser Agent
  → Browser Agent provides feedback
  → Supervisor coordinates fixes
```

---

## 📋 Common Tasks

### Add New Feature
1. Describe to Supervisor
2. Rust Agent implements
3. UI/UX Agent styles
4. Browser Agent reviews
5. Code Review Agent reviews
6. Supervisor approves

### Fix Bug
1. Describe to Supervisor
2. Rust Agent fixes
3. Testing Agent verifies
4. Browser Agent tests
5. Supervisor approves

### Update UI
1. Describe to Supervisor
2. UI/UX Agent updates
3. Browser Agent reviews
4. Supervisor approves

### Optimize Performance
1. Describe to Supervisor
2. Browser Agent identifies issues
3. Rust Agent optimizes
4. Browser Agent verifies
5. Supervisor approves

---

## 🎯 Quality Checklist

Before considering work complete:
- [ ] Code compiles (`cargo check`)
- [ ] No linter warnings (`cargo clippy`)
- [ ] Visual review passed (Browser Agent)
- [ ] Code review passed (Code Review Agent)
- [ ] Tests pass (Testing Agent)
- [ ] Supervisor approved

---

## 📞 Escalation

**Blockers**: Report to Supervisor immediately  
**Questions**: Ask Supervisor for guidance  
**Approvals**: Supervisor makes final decisions

---

## 📚 Documentation

- **Full Plan**: `LAUNCH_PLAN_1WEEK.md`
- **OKRs**: `AGENT_OKRS.md`
- **Workflows**: `AGENT_WORKFLOW.md`
- **Coordination**: `TASK_COORDINATION.md`
- **CEO Review**: `CEO_REVIEW.md`

---

**Last Updated**: 2026-01-15
