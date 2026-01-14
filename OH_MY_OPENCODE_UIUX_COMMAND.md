# Oh My OpenCode UI/UX Development Command

## 🎯 Complete Command for oh-my-opencode

Copy and paste this entire command into OpenCode:

```
ultrawork: Implement a complete UI/UX design system and optimize the entire application's frontend based on the design system documentation in `.cursor/rules/features/ui-ux-design-system.mdc`. 

The task includes:

1. **Design System Implementation**:
   - Ensure all components follow the color system (Bell Crab theme with primary #FA8A59, brand blue #6A7CED, brand purple #9E75E9)
   - Apply consistent spacing system (8px base scale)
   - Use proper border radius tokens (sm: 12px, md: 16px, lg: 24px, xl: 32px)
   - Implement shadow hierarchy (shadow, shadow-lg, shadow-soft)

2. **App Page UI Optimization** (Priority: High):
   - Optimize the reminder app page (`src/components/reminder_app.rs`) UI/UX
   - Improve visual hierarchy and spacing
   - Enhance header design with better typography and button placement
   - Optimize controls section (search + sort) for mobile responsiveness
   - Improve view switcher and filter tabs with better active states and icons
   - Enhance reminder cards with better hover effects and visual feedback
   - Optimize forms (AddReminderForm, EditReminderForm) with better validation feedback
   - Improve empty state design
   - Enhance statistics display with better card styling

3. **Responsive Design** (Priority: High):
   - Ensure all components work perfectly on mobile (375px, 414px)
   - Fix any overflow issues
   - Optimize touch interactions
   - Stack controls vertically on mobile
   - Ensure proper spacing for mobile viewports
   - Test and fix tablet viewports (768px, 1024px)

4. **Accessibility Enhancements** (Priority: Critical):
   - Verify all touch targets are ≥ 48x48px (measure in DevTools)
   - Add missing ARIA labels to all interactive elements
   - Enhance focus indicators with better visibility
   - Add live regions for dynamic content updates
   - Verify color contrast ratios (WCAG AA: 4.5:1 for text)
   - Add skip navigation link for keyboard users
   - Improve screen reader announcements

5. **CSS Organization**:
   - Update `assets/css/app.css` with optimized styles
   - Update `assets/css/responsive.css` with mobile-first breakpoints
   - Ensure all styles use CSS variables from `base.css`
   - Add missing utility classes if needed
   - Optimize animations for performance (use transform/opacity only)

6. **Component Styling**:
   - Review and optimize all components in `src/components/`
   - Ensure consistent styling across ListView, CardView, FolderView, CalendarView
   - Optimize form components (forms.rs)
   - Enhance modal components (modals.rs)
   - Improve card components (cards.rs)

7. **Performance Optimization**:
   - Optimize CSS animations (use transform and opacity only)
   - Add will-change where appropriate
   - Reduce animation complexity
   - Ensure no layout shifts

8. **Lighthouse 100% Compliance** (MANDATORY):
   - Verify all touch targets ≥ 48x48px
   - Ensure HTML lang attribute is set
   - Verify meta description exists
   - Check ARIA labels on all interactive elements
   - Verify semantic HTML structure
   - Run Lighthouse audit and fix any issues

**Requirements**:
- Follow the design system in `.cursor/rules/features/ui-ux-design-system.mdc`
- Maintain 100% Lighthouse scores (Performance, Accessibility, Best Practices, SEO)
- Use CSS variables for all colors and spacing
- Follow mobile-first responsive design
- All touch targets must be ≥ 48x48px
- Test on mobile viewports (375px, 414px, 768px)
- Ensure smooth 60fps animations
- No console errors
- All code must compile and pass clippy checks

**Reference Files**:
- Design System: `.cursor/rules/features/ui-ux-design-system.mdc`
- App Page Plan: `APP_PAGE_UI_OPTIMIZATION_PLAN.md`
- Lighthouse Standards: `.cursor/rules/core/lighthouse-100-standards.mdc`
- Code Standards: `.cursor/rules/core/code-standards-100.mdc`
- Current CSS: `assets/css/app.css`, `assets/css/responsive.css`
- Current Components: `src/components/reminder_app.rs`, `src/components/forms.rs`, `src/components/cards.rs`

**Deliverables**:
1. Optimized app page UI with improved visual hierarchy
2. Fully responsive design for all viewports
3. Enhanced accessibility (100% Lighthouse accessibility score)
4. Optimized CSS with proper use of design tokens
5. All components following design system patterns
6. Performance optimizations (60fps animations)
7. Lighthouse audit passing 100% in all categories

Start by reading the design system documentation, then systematically optimize each component and style file.
```

## 📋 Alternative: Step-by-Step Commands

If you prefer to break it down into smaller tasks:

### Step 1: Design System Review
```
ultrawork: Review the design system in `.cursor/rules/features/ui-ux-design-system.mdc` and create a summary of all design tokens, color patterns, spacing system, and responsive breakpoints. Verify that all CSS files are using these tokens correctly.
```

### Step 2: App Page UI Optimization
```
ultrawork: Optimize the reminder app page UI based on APP_PAGE_UI_OPTIMIZATION_PLAN.md. Focus on visual hierarchy, spacing, and component styling. Ensure all styles use CSS variables from the design system.
```

### Step 3: Responsive Design
```
ultrawork: Implement mobile-first responsive design for the entire app page. Fix all overflow issues, optimize touch interactions, and ensure proper spacing on mobile viewports (375px, 414px, 768px). Test and verify all breakpoints work correctly.
```

### Step 4: Accessibility
```
ultrawork: Enhance accessibility for the app page. Verify all touch targets ≥ 48x48px, add missing ARIA labels, enhance focus indicators, and ensure WCAG AA compliance. Run Lighthouse audit and fix all accessibility issues.
```

### Step 5: Performance & Final Polish
```
ultrawork: Optimize CSS animations for performance, ensure 60fps animations, fix any layout shifts, and run final Lighthouse audit. Verify 100% scores in all categories.
```

## 🚀 Quick Start

1. **Open OpenCode**:
   ```bash
   cd /Users/klaus_mac/Desktop/vibe_code/remind-me-pwa
   opencode
   ```

2. **Paste the complete command above** (the `ultrawork:` command)

3. **Let oh-my-opencode work** - it will:
   - Read the design system documentation
   - Analyze current CSS and components
   - Implement optimizations systematically
   - Test and verify changes

## 📝 Notes

- The command uses `ultrawork` keyword for maximum performance
- oh-my-opencode will orchestrate multiple agents (Sisyphus, Oracle, Frontend Engineer)
- It will read your `.cursor/rules/` for project context
- It will follow your project's patterns and conventions
- All changes will maintain Lighthouse 100% scores

---

**Last Updated**: 2026-01-14
