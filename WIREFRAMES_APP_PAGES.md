# Remind Me PWA – App Wireframes (Markdown)

This document describes the **app screens and UI elements** shown in the provided designs.  
It lists **each screen**, the **layout blocks**, and **every button/navigation/interactive UI** with intended behavior.

---

## Progress (Current Implementation)

- **Dashboard**: Search, filters, stats, reminder list, cards, header actions, and FAB done
- **Calendar**: Month + week modes done; selected-day + unscheduled lists done; drag & drop reschedule (desktop + touch) done
- **Categories / Tags**: Tag manager UI done (add/edit/delete, reorder); touch drag reorder done
- **Reminder Detail**: Detail modal with edit/complete/delete actions done
- **Bottom Tab Bar & Settings**: Bottom navigation + Settings screen done

---

## Global Navigation (Bottom Tab Bar)

Visible on most app pages.

- **Home** (icon + label)
  - Action: Navigate to **Dashboard / Reminders**.
- **Events** (icon + label)
  - Action: Navigate to **Calendar**.
- **Groups** (icon + label)
  - Action: Navigate to **Categories / Tags**.
- **Settings** (icon + label)
  - Action: Navigate to **Settings** (not shown in wireframes).

---

## Screen 1 — Dashboard (Mobile PWA)

**Purpose:** Overview of reminders with quick filters and stats.

### Layout Blocks
1. **Top App Bar**
   - App icon + app name (“RemindMe”)
   - Profile/avatar button
2. **Search Bar**
3. **Filter Chips** (horizontal)
4. **Quick Stats Cards** (2 cards)
5. **Reminders List**
6. **Floating Action Button (FAB)**
7. **Bottom Tab Bar**

### UI Elements & Actions
- **Avatar/Profile Button**
  - Action: Open profile or user menu.
- **Search Input**
  - Placeholder: “Search reminders...”
  - Action: Filter list by query.
- **Filter Chips**
  - **All** (active)
  - **Today**
  - **Upcoming**
  - Action: Filter list by timeframe.
- **Stats Card: Done Today**
  - Displays count (e.g., “12”)
  - Action: Optional filter to completed today.
- **Stats Card: Pending**
  - Displays count (e.g., “4”)
  - Action: Optional filter to pending.
- **Section Title: Reminders**
  - **Sort Button** (text/icon)
  - Action: Open sort options (date, priority, category).
- **Reminder List Item Card**
  - **Category Tag** (e.g., WORK, PERSONAL)
  - **Title**
  - **Meta Row** (icon + “Today”, icon + time)
  - **Status/Indicator Dot**
  - Action: Tap to open **Reminder Detail**.
- **Floating “+” Button**
  - Action: Create new reminder (open **Add Reminder**).

---

## Screen 2 — Add Reminder

**Purpose:** Create a new reminder with title, details, date/time, priority, category.

### Layout Blocks
1. **Top Bar** (back)
2. **Form: Reminder Title**
3. **Notes & Details**
4. **Date + Time**
5. **Priority Level**
6. **Category Pills**
7. **Primary CTA**

### UI Elements & Actions
- **Back Button**
  - Action: Return to previous screen.
- **Title Input**
  - Label: “Reminder Title”
  - Placeholder: example text
  - Action: Capture reminder title.
- **Notes Text Area**
  - Label: “Notes & Details”
  - Action: Capture description.
- **Date Picker**
  - Action: Select date.
- **Time Picker**
  - Action: Select time.
- **Priority Level Chips**
  - **Low**, **Med**, **High**
  - Action: Select one priority.
- **Category Chips**
  - Examples: Personal, Work, Shopping
  - **Add New** (link/button)
  - Action: Assign category or create new.
- **Save Reminder Button**
  - Primary CTA
  - Action: Create reminder and return to list.

---

## Screen 3 — Reminder Detail

**Purpose:** View reminder countdown, metadata, and actions.

### Layout Blocks
1. **Top Bar** (back + title + more)
2. **Countdown / Timer Card**
3. **Info Cards** (priority, category, schedule)
4. **Notes**
5. **Action Buttons**

### UI Elements & Actions
- **Back Button**
  - Action: Return to previous screen.
- **Title: Reminder Name**
  - Example: “Board Presentation”
- **Overflow / More Button**
  - Action: Open menu (edit, delete, share).
- **Countdown Ring + Time**
  - Displays time remaining.
- **Priority Badge**
  - Example: High.
- **Category Badge**
  - Example: Work.
- **Schedule Info**
  - Date/time line
  - “Remind me …” line
- **Notes Section**
  - Read-only text display.
- **Edit Button**
  - Action: Open edit form.
- **Mark Done Button**
  - Action: Mark reminder as completed.

---

## Screen 4 — Calendar View

**Purpose:** View reminders in calendar layout by date.

### Layout Blocks
1. **Month Header** (month name + arrows)
2. **Calendar Grid**
3. **Month/Week Toggle**
4. **Selected Date Summary**
5. **Agenda List**
6. **Bottom Tab Bar**

### UI Elements & Actions
- **Month Selector**
  - **Left Arrow** (previous month)
  - **Right Arrow** (next month)
  - Month label dropdown
  - Action: Change month.
- **Calendar Day Cells**
  - Action: Select date.
  - Indicators: dots or highlights for reminders.
- **Month / Week Toggle**
  - Action: Switch calendar mode.
- **Selected Date Header**
  - Example: “Friday, Dec 15”
  - **Tag** (e.g., “3 REMINDERS”)
- **Agenda Items**
  - Time, title, category tag, location
  - Action: Tap to open reminder detail.

---

## Screen 5 — Categories Manager

**Purpose:** Manage categories/tags, reorder, and delete.

### Layout Blocks
1. **Top Header** (title + add button)
2. **Instruction Text** (drag to reorder)
3. **Category List**
4. **Add Category Button**
5. **Bottom Tab Bar**

### UI Elements & Actions
- **Add “+” Button**
  - Action: Create new category.
- **Category Card**
  - **Icon**
  - **Category Name**
  - **Count** (e.g., “12 reminders”)
  - **Drag Handle** (reorder)
  - **Delete Button** (trash icon) — appears on swipe or active state
  - Action: Tap to view category reminders.
- **Create New Category Button**
  - Action: Open add category form.

---

## Interaction & Accessibility Notes

- All interactive elements must be **≥ 48x48px** touch targets.
- Provide **ARIA labels** for all icon-only buttons.
- Ensure **focus indicators** are visible on keyboard navigation.
- Keep **color contrast** at WCAG AA (≥ 4.5:1 for text).

---

## Screen Map (Navigation Flow)

```text
Dashboard -> Add Reminder
Dashboard -> Reminder Detail
Dashboard -> Calendar
Dashboard -> Categories
Calendar -> Reminder Detail
Categories -> Category List -> Reminder Detail
```

---

## Optional Enhancements (Future)

- Swipe actions on reminder cards (complete, delete)
- Quick add from Dashboard search bar
- Bulk actions in Categories list
- Advanced filters (tags, priority, date range)

