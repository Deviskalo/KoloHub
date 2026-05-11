pub const TOKENS_CSS: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../design-system/tokens.css"
));

pub const APP_CSS: &str = r#"
.kh-preview-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1rem;
  align-items: stretch;
}
.kh-dashboard-grid {
  display: grid;
  grid-template-columns: 1fr;
  gap: 1rem;
}
.kh-topbar-nav {
  display: grid;
  grid-template-columns: minmax(0, 1fr) auto;
  align-items: center;
  gap: 0.55rem;
}
.kh-main-shell {
  max-width: 72rem;
  margin: 0 auto;
  padding: 10.25rem 1rem 2rem;
}
.kh-brand-group {
  display: flex;
  align-items: center;
  gap: 0.6rem;
  min-width: 0;
}
.kh-desktop-sep {
  display: none;
}
.kh-nav-links {
  display: none;
  align-items: center;
  gap: 0.2rem;
  overflow-x: auto;
  max-width: 100%;
}
.kh-top-controls {
  display: flex;
  align-items: center;
  gap: 0.4rem;
}
.kh-mobile-menu-btn {
  height: 2.5rem;
  width: 2.5rem;
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border-subtle);
  background: var(--color-surface-elevated);
  color: var(--color-text-primary);
  font-size: 1.05rem;
  line-height: 1;
}
.kh-mobile-backdrop {
  position: fixed;
  inset: 0;
  background: rgb(2 6 23 / 0.36);
  z-index: 70;
}
.kh-mobile-drawer {
  position: fixed;
  top: 0;
  right: 0;
  width: min(84vw, 340px);
  height: 100dvh;
  z-index: 80;
  background: var(--color-surface-elevated);
  border-left: 1px solid var(--color-border-subtle);
  box-shadow: -14px 0 35px rgb(0 0 0 / 0.24);
  padding: 0.95rem 0.75rem;
  display: grid;
  grid-template-rows: auto 1fr auto;
  gap: 0.7rem;
  animation: kh-drawer-in 180ms ease-out;
}
.kh-mobile-drawer-head {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 0.6rem;
}
.kh-mobile-drawer-title {
  margin: 0;
  color: var(--color-text-primary);
  font-size: var(--text-base);
  font-weight: var(--font-weight-semibold);
}
@keyframes kh-drawer-in {
  from {
    transform: translateX(40px);
    opacity: 0.6;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}
.kh-actions-row {
  display: none;
  gap: 0.5rem;
  margin-left: auto;
}
.kh-actions-row a { white-space: nowrap; }
.kh-mobile-menu-panel {
  grid-column: 1 / -1;
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-lg);
  background: var(--color-surface-elevated);
  padding: 0.65rem;
  display: grid;
  gap: 0.5rem;
}
.kh-mobile-menu-links {
  display: grid;
  grid-template-columns: 1fr;
  grid-auto-rows: min-content;
  gap: 0.5rem;
  align-content: start;
  align-items: start;
  overflow-y: auto;
  min-height: 0;
}
.kh-mobile-menu-link {
  display: block;
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  padding: 0.62rem 0.78rem;
  color: var(--color-text-primary);
  text-decoration: none;
  font-size: var(--text-sm);
  line-height: 1.2;
  font-weight: var(--font-weight-medium);
  background: var(--color-surface);
  box-shadow: 0 1px 0 rgb(0 0 0 / 0.03);
  transition: transform 120ms ease, border-color 120ms ease, background-color 120ms ease;
}
.kh-mobile-menu-link:hover {
  border-color: var(--color-border-strong);
  background: var(--color-brand-muted);
  transform: translateY(-1px);
}
.kh-mobile-menu-link:first-child {
  border-color: var(--color-border-strong);
  background: var(--color-brand-muted);
  color: var(--color-brand-foreground);
}
.kh-mobile-menu-actions {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 0.45rem;
  margin-bottom: 0.6rem;
}
.kh-mobile-menu-actions .kh-btn-brand,
.kh-mobile-menu-actions .kh-btn-outline {
  height: 2.35rem;
  padding: 0 0.65rem;
  font-size: var(--text-xs);
  border-radius: var(--radius-md);
  justify-content: center;
}
.kh-preview-image {
  width: 100%;
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-card);
  border: 1px solid var(--color-border-subtle);
  background: var(--color-surface-elevated);
  object-fit: contain;
}
.kh-customer-flow {
  max-width: 72rem;
  margin: 0 auto;
  padding: 0 1rem 1.5rem;
}
.kh-customer-flow .track {
  display: grid;
  grid-template-columns: 1fr;
  gap: 0.55rem;
}
.kh-customer-flow .step {
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  background: var(--color-surface-elevated);
  padding: 0.6rem 0.7rem;
}
.kh-customer-flow .step .label {
  margin: 0;
  color: var(--color-text-primary);
  font-size: var(--text-sm);
  font-weight: var(--font-weight-semibold);
}
.kh-customer-flow .step .meta {
  margin: 0.18rem 0 0;
  color: var(--color-text-secondary);
  font-size: var(--text-xs);
}
.kh-stat-grid {
  display: grid;
  grid-template-columns: repeat(3, minmax(0, 1fr));
  gap: 0.5rem;
}
.kh-stat-chip {
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  padding: 0.45rem 0.55rem;
  min-height: 3.05rem;
}
.kh-stat-chip .label {
  margin: 0;
  font-size: var(--text-xs);
  color: var(--color-text-muted);
}
.kh-stat-chip .value {
  margin: 0.1rem 0 0;
  font-size: var(--text-base);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
}
@media (max-width: 480px) {
  .kh-mobile-menu-actions { grid-template-columns: 1fr; }
  .kh-brand-group a {
    font-size: var(--text-base) !important;
  }
}
@media (min-width: 980px) {
  .kh-main-shell {
    padding-top: 7rem;
  }
  .kh-topbar-nav {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    justify-content: space-between;
    gap: 0.75rem;
  }
  .kh-brand-group {
    min-width: unset;
  }
  .kh-desktop-sep {
    display: block;
    width: 1px;
    height: 1.75rem;
    background: var(--color-border-subtle);
    flex: 0 0 auto;
  }
  .kh-nav-links {
    display: flex;
  }
  .kh-top-controls .kh-mobile-menu-btn {
    display: none;
  }
  .kh-mobile-backdrop,
  .kh-mobile-drawer {
    display: none;
  }
  .kh-actions-row {
    display: flex;
  }
  .kh-preview-grid {
    grid-template-columns: minmax(280px, 0.9fr) minmax(0, 1.7fr);
    gap: 1.25rem;
  }
  .kh-dashboard-grid {
    grid-template-columns: 1fr 1fr;
  }
  .kh-customer-flow .track {
    grid-template-columns: repeat(5, minmax(0, 1fr));
    gap: 0.7rem;
  }
}
"#;
