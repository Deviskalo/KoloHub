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
.kh-hero-banner {
  background-image: url('/images/banner-mobile.jpeg');
  background-size: cover;
  background-position: center;
  position: relative;
}
.kh-hero-banner::before {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(to bottom, rgba(0,0,0,0.2), rgba(0,0,0,0.7));
  border-radius: inherit;
  z-index: 0;
}
.kh-hero-banner > * {
  position: relative;
  z-index: 1;
}
.kh-hero-banner h1, .kh-hero-banner p, .kh-hero-banner .kh-text-muted {
  color: #ffffff !important;
  text-shadow: 0 2px 4px rgba(0,0,0,0.5);
}
@media (min-width: 980px) {
  .kh-hero-banner {
    background-image: url('/images/banner-desktop.jpeg');
  }
}
.kh-product-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 1rem;
  margin-top: 1.5rem;
}
.kh-product-card {
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-lg);
  background: var(--color-surface);
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: transform 120ms ease, box-shadow 120ms ease;
  cursor: pointer;
  text-decoration: none;
}
.kh-product-card:hover {
  transform: translateY(-2px);
  box-shadow: var(--shadow-card);
  border-color: var(--color-border-strong);
}
.kh-product-image {
  width: 100%;
  aspect-ratio: 1;
  background: var(--color-surface-elevated);
  object-fit: cover;
  border-bottom: 1px solid var(--color-border-subtle);
}
.kh-product-details {
  padding: 0.75rem;
  display: flex;
  flex-direction: column;
  flex: 1;
}
.kh-product-title {
  font-size: var(--text-sm);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
  margin: 0 0 0.2rem 0;
  line-height: 1.3;
}
.kh-product-vendor {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
  margin: 0 0 0.5rem 0;
}
.kh-product-footer {
  margin-top: auto;
  display: flex;
  align-items: center;
  justify-content: space-between;
}
.kh-product-price {
  font-size: var(--text-base);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0;
}
.kh-add-btn {
  height: 2rem;
  width: 2rem;
  border-radius: var(--radius-full);
  background: var(--color-brand);
  color: #fff;
  border: none;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1.25rem;
  font-weight: bold;
  cursor: pointer;
}
.kh-add-btn:hover {
  background: var(--color-brand-hover);
}
.kh-category-strip {
  display: flex;
  gap: 0.5rem;
  overflow-x: auto;
  padding-bottom: 0.5rem;
  margin: 1.5rem 0 0;
  scrollbar-width: none;
}
.kh-category-strip::-webkit-scrollbar {
  display: none;
}
.kh-category-chip {
  padding: 0.4rem 1rem;
  border-radius: var(--radius-full);
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  color: var(--color-text-secondary);
  font-size: var(--text-sm);
  font-weight: var(--font-weight-medium);
  white-space: nowrap;
  cursor: pointer;
  transition: all 150ms ease;
}
.kh-category-chip.active {
  background: var(--color-text-primary);
  color: var(--color-surface);
  border-color: var(--color-text-primary);
}
@media (min-width: 640px) {
  .kh-product-grid { grid-template-columns: repeat(3, minmax(0, 1fr)); }
}
@media (min-width: 980px) {
  .kh-product-grid { grid-template-columns: repeat(5, minmax(0, 1fr)); }
}
.kh-pdp-layout {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
.kh-pdp-image-container {
  width: 100%;
  aspect-ratio: 1;
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  overflow: hidden;
  box-shadow: var(--shadow-card);
}
.kh-pdp-image {
  width: 100%;
  height: 100%;
  object-fit: cover;
}
.kh-pdp-content {
  display: flex;
  flex-direction: column;
  gap: 1.25rem;
}
.kh-pdp-header {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}
.kh-pdp-title {
  margin: 0;
  font-size: var(--text-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  line-height: 1.2;
}
.kh-pdp-vendor {
  margin: 0;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}
.kh-pdp-price {
  margin: 0;
  font-size: var(--text-3xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-brand);
}
.kh-pdp-badge {
  display: inline-flex;
  align-items: center;
  padding: 0.35rem 0.75rem;
  border-radius: var(--radius-full);
  background: var(--color-accent-muted);
  color: var(--color-accent);
  font-size: var(--text-xs);
  font-weight: var(--font-weight-semibold);
  width: max-content;
}
.kh-pdp-desc {
  margin: 0;
  font-size: var(--text-base);
  color: var(--color-text-secondary);
  line-height: 1.6;
}
.kh-pdp-cta-wrap {
  position: fixed;
  bottom: 0;
  left: 0;
  width: 100%;
  padding: 1rem;
  background: var(--color-surface-elevated);
  border-top: 1px solid var(--color-border-subtle);
  box-shadow: 0 -4px 20px rgba(0,0,0,0.08);
  z-index: 10;
}
.kh-btn-lg {
  height: 3.5rem;
  width: 100%;
  border-radius: var(--radius-lg);
  font-size: var(--text-lg);
}

@media (min-width: 768px) {
  .kh-pdp-layout {
    flex-direction: row;
    gap: 3rem;
    align-items: flex-start;
  }
  .kh-pdp-image-container {
    flex: 1;
    position: sticky;
    top: 6rem;
  }
  .kh-pdp-content {
    flex: 1;
    padding-top: 1rem;
  }
  .kh-pdp-cta-wrap {
    position: static;
    padding: 0;
    box-shadow: none;
    background: transparent;
    border: none;
    margin-top: 1.5rem;
  }
}
.kh-cart-layout {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
.kh-cart-items {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
.kh-cart-item {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background: var(--color-surface);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-lg);
}
.kh-cart-item-img {
  width: 5rem;
  height: 5rem;
  border-radius: var(--radius-md);
  object-fit: cover;
  background: var(--color-surface-elevated);
}
.kh-cart-item-info {
  flex: 1;
  display: flex;
  flex-direction: column;
}
.kh-cart-item-title {
  margin: 0 0 0.25rem;
  font-size: var(--text-base);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
}
.kh-cart-item-price {
  margin: 0 0 0.5rem;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}
.kh-stepper {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: auto;
}
.kh-stepper-btn {
  width: 2rem;
  height: 2rem;
  border-radius: var(--radius-md);
  border: 1px solid var(--color-border-subtle);
  background: var(--color-surface-elevated);
  color: var(--color-text-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
}
.kh-stepper-val {
  font-weight: var(--font-weight-semibold);
  font-size: var(--text-sm);
  min-width: 1rem;
  text-align: center;
}
.kh-summary-card {
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  padding: 1.5rem;
  box-shadow: var(--shadow-card);
}
.kh-summary-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.75rem;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}
.kh-summary-total {
  display: flex;
  justify-content: space-between;
  margin-top: 1rem;
  padding-top: 1rem;
  border-top: 1px solid var(--color-border-subtle);
  font-size: var(--text-lg);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
}

@media (min-width: 768px) {
  .kh-cart-layout {
    flex-direction: row;
    gap: 3rem;
  }
  .kh-cart-items {
    flex: 1.5;
  }
  .kh-summary-card {
    flex: 1;
    position: sticky;
    top: 6rem;
    height: fit-content;
  }
}
.kh-checkout-layout {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
.kh-form-section {
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  padding: 1.5rem;
  box-shadow: var(--shadow-card);
}
.kh-form-section-title {
  margin: 0 0 1rem;
  font-size: var(--text-lg);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
}
.kh-form-group {
  margin-bottom: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.35rem;
}
.kh-form-label {
  font-size: var(--text-sm);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-secondary);
}
.kh-input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  color: var(--color-text-primary);
  font-size: var(--text-base);
  font-family: inherit;
  transition: border-color 150ms;
}
.kh-input:focus {
  outline: none;
  border-color: var(--color-brand);
}
.kh-payment-options {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}
.kh-payment-opt {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 1rem;
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-md);
  background: var(--color-surface);
  cursor: pointer;
  transition: all 150ms ease;
}
.kh-payment-opt.active {
  border-color: var(--color-brand);
  background: var(--color-brand-muted);
}
.kh-payment-opt-circle {
  width: 1.25rem;
  height: 1.25rem;
  border-radius: 50%;
  border: 2px solid var(--color-border-subtle);
  display: flex;
  align-items: center;
  justify-content: center;
}
.kh-payment-opt.active .kh-payment-opt-circle::after {
  content: "";
  width: 0.65rem;
  height: 0.65rem;
  border-radius: 50%;
  background: var(--color-brand);
}
.kh-payment-opt-label {
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
}
.kh-payment-opt-desc {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
}

@media (min-width: 768px) {
  .kh-checkout-layout {
    flex-direction: row;
    gap: 3rem;
  }
  .kh-checkout-main {
    flex: 1.5;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }
}
.kh-track-layout {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
.kh-track-card {
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  padding: 1.5rem;
  box-shadow: var(--shadow-card);
}
.kh-track-header {
  margin: 0 0 1.25rem;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--color-border-subtle);
}
.kh-track-header h2 {
  margin: 0;
  font-size: var(--text-lg);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
}
.kh-track-header p {
  margin: 0.25rem 0 0;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}
.kh-timeline {
  display: flex;
  flex-direction: column;
  gap: 0;
}
.kh-timeline-step {
  display: flex;
  gap: 1rem;
  position: relative;
  padding-bottom: 2rem;
}
.kh-timeline-step:last-child {
  padding-bottom: 0;
}
.kh-timeline-icon {
  width: 2rem;
  height: 2rem;
  border-radius: 50%;
  background: var(--color-surface);
  border: 2px solid var(--color-border-subtle);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 1rem;
  z-index: 2;
  flex-shrink: 0;
}
.kh-timeline-step.done .kh-timeline-icon {
  border-color: var(--color-brand);
  background: var(--color-brand);
  color: #fff;
}
.kh-timeline-step.active .kh-timeline-icon {
  border-color: var(--color-brand);
  background: var(--color-brand-muted);
  color: var(--color-brand);
  box-shadow: 0 0 0 4px rgba(var(--color-brand-rgb), 0.1);
}
.kh-timeline-line {
  position: absolute;
  top: 2rem;
  left: 1rem;
  bottom: 0;
  width: 2px;
  background: var(--color-border-subtle);
  transform: translateX(-50%);
  z-index: 1;
}
.kh-timeline-step:last-child .kh-timeline-line {
  display: none;
}
.kh-timeline-step.done .kh-timeline-line {
  background: var(--color-brand);
}
.kh-timeline-content {
  flex: 1;
  padding-top: 0.25rem;
}
.kh-timeline-title {
  margin: 0;
  font-size: var(--text-base);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
}
.kh-timeline-desc {
  margin: 0.25rem 0 0;
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
}

@media (min-width: 768px) {
  .kh-track-layout {
    flex-direction: row;
    align-items: flex-start;
  }
  .kh-track-layout > div {
    flex: 1;
  }
}
.kh-dashboard-shell {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
  padding-top: 5rem;
}
.kh-sidebar {
  display: none;
  width: 16rem;
  background: var(--color-surface-elevated);
  border-right: 1px solid var(--color-border-subtle);
  padding: 1.5rem 1rem;
  flex-direction: column;
  position: fixed;
  top: 5rem;
  bottom: 0;
  left: 0;
  z-index: 40;
}
.kh-sidebar-link {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.75rem 1rem;
  color: var(--color-text-secondary);
  text-decoration: none;
  border-radius: var(--radius-md);
  font-weight: var(--font-weight-medium);
  transition: all 150ms ease;
  margin-bottom: 0.25rem;
}
.kh-sidebar-link:hover {
  background: var(--color-surface);
  color: var(--color-text-primary);
}
.kh-sidebar-link.active {
  background: var(--color-brand-muted);
  color: var(--color-brand-foreground);
  font-weight: var(--font-weight-semibold);
  border-left: 3px solid var(--color-brand);
}
.kh-dashboard-main {
  flex: 1;
  padding: 1.5rem 1rem;
  max-width: 100%;
}

@media (min-width: 768px) {
  .kh-dashboard-shell {
    flex-direction: row;
    padding-top: 5rem;
  }
  .kh-sidebar {
    display: flex;
  }
  .kh-dashboard-main {
    margin-left: 16rem;
    padding: 2rem 2.5rem;
  }
}
.kh-dashboard-header {
  margin-bottom: 2rem;
}
.kh-dashboard-title {
  margin: 0;
  font-size: var(--text-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
}
.kh-stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 1.5rem;
  margin-bottom: 2rem;
}
.kh-stat-card {
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  padding: 1.5rem;
  box-shadow: var(--shadow-card);
}
.kh-stat-label {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  font-weight: var(--font-weight-medium);
  margin: 0 0 0.5rem;
}
.kh-stat-val {
  font-size: var(--text-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0;
}
.kh-stat-trend {
  display: inline-flex;
  align-items: center;
  gap: 0.25rem;
  font-size: var(--text-xs);
  font-weight: var(--font-weight-bold);
  padding: 0.15rem 0.5rem;
  border-radius: var(--radius-full);
  margin-top: 0.75rem;
}
.kh-stat-trend.up {
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
}
.kh-stat-trend.down {
  background: rgba(239, 68, 68, 0.1);
  color: #ef4444;
}

.kh-dash-row {
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
}
.kh-dash-card {
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  padding: 1.5rem;
  box-shadow: var(--shadow-card);
  flex: 1;
}
.kh-dash-card-header {
  font-size: var(--text-lg);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0 0 1rem;
  padding-bottom: 0.75rem;
  border-bottom: 1px solid var(--color-border-subtle);
}
.kh-dash-chart-placeholder {
  height: 20rem;
  display: flex;
  align-items: center;
  justify-content: center;
  background: var(--color-surface);
  border: 1px dashed var(--color-border-strong);
  border-radius: var(--radius-lg);
  color: var(--color-text-muted);
  font-size: var(--text-sm);
}
.kh-activity-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
.kh-activity-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-bottom: 1rem;
  border-bottom: 1px solid var(--color-border-subtle);
}
.kh-activity-item:last-child {
  border-bottom: none;
  padding-bottom: 0;
}
.kh-activity-info p { margin: 0; }
.kh-activity-title {
  font-size: var(--text-base);
  font-weight: var(--font-weight-medium);
  color: var(--color-text-primary);
}
.kh-activity-sub {
  font-size: var(--text-sm);
  color: var(--color-text-secondary);
  margin-top: 0.25rem !important;
}

.kh-data-table-wrapper {
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  box-shadow: var(--shadow-card);
  overflow-x: auto;
}
.kh-data-table {
  width: 100%;
  border-collapse: collapse;
  text-align: left;
  min-width: 600px;
}
.kh-data-table th {
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--color-border-subtle);
  font-size: var(--text-sm);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-secondary);
  background: var(--color-surface);
}
.kh-data-table td {
  padding: 1rem 1.5rem;
  border-bottom: 1px solid var(--color-border-subtle);
  font-size: var(--text-sm);
  color: var(--color-text-primary);
  vertical-align: middle;
}
.kh-data-table tr:last-child td {
  border-bottom: none;
}
.kh-data-table tbody tr:hover {
  background: rgba(0,0,0,0.02);
}
.kh-table-actions {
  display: flex;
  gap: 0.5rem;
}
.kh-badge-status {
  display: inline-flex;
  padding: 0.2rem 0.6rem;
  border-radius: var(--radius-full);
  font-size: var(--text-xs);
  font-weight: var(--font-weight-bold);
}
.kh-badge-status.success {
  background: rgba(16, 185, 129, 0.1);
  color: #10b981;
}
.kh-badge-status.warning {
  background: rgba(245, 158, 11, 0.1);
  color: #f59e0b;
}

@media (min-width: 980px) {
  .kh-dash-row {
    flex-direction: row;
  }
  .kh-dash-row > :first-child {
    flex: 2;
  }
  .kh-dash-row > :last-child {
    flex: 1;
  }
}
.kh-storefront-hero {
  position: relative;
  border-radius: var(--radius-xl);
  overflow: hidden;
  margin-bottom: 2rem;
  background-image: url('/images/banner-desktop.jpeg');
  background-size: cover;
  background-position: center;
  min-height: 24rem;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  padding: 2rem;
}
.kh-storefront-hero::before {
  content: "";
  position: absolute;
  inset: 0;
  background: linear-gradient(0deg, rgba(0,0,0,0.7) 0%, rgba(0,0,0,0.3) 100%);
  z-index: 1;
}
.kh-storefront-content {
  position: relative;
  z-index: 2;
  color: #fff;
  max-width: 42rem;
}
.kh-storefront-title {
  font-size: 2.5rem;
  font-weight: var(--font-weight-bold);
  margin: 0 0 1rem;
  line-height: 1.1;
}
.kh-storefront-sub {
  font-size: var(--text-lg);
  margin: 0 0 2rem;
  opacity: 0.9;
}
.kh-search-wrapper {
  display: flex;
  background: var(--color-surface);
  border-radius: var(--radius-full);
  padding: 0.5rem;
  box-shadow: var(--shadow-lg);
  max-width: 32rem;
  margin: 0 auto;
}
.kh-search-input {
  flex: 1;
  background: transparent;
  border: none;
  font-size: var(--text-base);
  color: var(--color-text-primary);
  padding: 0.5rem 1rem;
  outline: none;
}
.kh-search-btn {
  background: var(--color-brand);
  color: #fff;
  border: none;
  border-radius: var(--radius-full);
  padding: 0 1.5rem;
  font-weight: var(--font-weight-bold);
  cursor: pointer;
  transition: opacity 150ms ease;
}
.kh-search-btn:hover { opacity: 0.9; }

.kh-cat-scroll {
  display: flex;
  gap: 1rem;
  overflow-x: auto;
  padding-bottom: 1rem;
  margin-bottom: 2rem;
  scrollbar-width: none;
}
.kh-cat-scroll::-webkit-scrollbar { display: none; }
.kh-cat-card {
  flex: 0 0 auto;
  width: 120px;
  height: 120px;
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-lg);
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  text-decoration: none;
  color: var(--color-text-primary);
  font-weight: var(--font-weight-semibold);
  transition: transform 150ms ease, border-color 150ms ease;
  cursor: pointer;
}
.kh-cat-card:hover {
  transform: translateY(-2px);
  border-color: var(--color-brand);
}
.kh-cat-icon {
  font-size: 2rem;
}
.kh-cat-label {
  font-size: var(--text-sm);
}

.kh-vendor-cta {
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  padding: 2.5rem;
  text-align: center;
  margin-top: 4rem;
}
.kh-vendor-cta h2 {
  font-size: var(--text-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
  margin: 0 0 1rem;
}
.kh-vendor-cta p {
  color: var(--color-text-secondary);
  max-width: 38rem;
  margin: 0 auto 2rem;
  line-height: 1.5;
}

.kh-footer {
  background: var(--color-surface-elevated);
  border-top: 1px solid var(--color-border-subtle);
  margin-top: 4rem;
  padding: 4rem 2rem 2rem;
  font-family: var(--font-body);
}
.kh-footer-top {
  display: grid;
  grid-template-columns: 1fr;
  gap: 3rem;
  max-width: 1200px;
  margin: 0 auto;
  padding-bottom: 3rem;
  border-bottom: 1px solid var(--color-border-subtle);
}
@media (min-width: 768px) {
  .kh-footer-top {
    grid-template-columns: 2fr 1fr 1fr 1fr;
  }
}
.kh-footer-brand h2 {
  font-size: var(--text-2xl);
  font-weight: var(--font-weight-bold);
  color: var(--color-brand);
  margin: 0 0 1rem;
}
.kh-footer-brand p {
  color: var(--color-text-secondary);
  line-height: 1.5;
  font-size: var(--text-sm);
  max-width: 20rem;
}
.kh-footer-links {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}
.kh-footer-links h3 {
  font-size: var(--text-base);
  font-weight: var(--font-weight-semibold);
  color: var(--color-text-primary);
  margin: 0 0 0.5rem;
}
.kh-footer-links a {
  color: var(--color-text-secondary);
  text-decoration: none;
  font-size: var(--text-sm);
  transition: color 150ms ease;
}
.kh-footer-links a:hover {
  color: var(--color-brand);
}
.kh-footer-bottom {
  max-width: 1200px;
  margin: 2rem auto 0;
  display: flex;
  flex-direction: column;
  gap: 1rem;
  justify-content: space-between;
  align-items: center;
  color: var(--color-text-muted);
  font-size: var(--text-xs);
}
@media (min-width: 640px) {
  .kh-footer-bottom {
    flex-direction: row;
  }
}
.kh-footer-legal {
  display: flex;
  gap: 1.5rem;
}
.kh-footer-legal a {
  color: var(--color-text-muted);
  text-decoration: none;
}
.kh-footer-legal a:hover {
  color: var(--color-text-secondary);
}

@media (max-width: 768px) {
  .kh-storefront-hero {
    min-height: 20rem;
    padding: 1.5rem;
  }
  .kh-storefront-title {
    font-size: 2rem;
  }
}

/* ─── Micro-animations ─── */
@keyframes kh-fade-in-up {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
.kh-animate-fade-in-up {
  animation: kh-fade-in-up 0.6s ease forwards;
}

/* ─── Advanced Search Dropdown ─── */
.kh-search-container {
  position: relative;
  max-width: 32rem;
  margin: 0 auto;
}
.kh-search-dropdown {
  position: absolute;
  top: calc(100% + 0.5rem);
  left: 0;
  right: 0;
  background: var(--color-surface-glass);
  -webkit-backdrop-filter: blur(var(--blur-glass));
  backdrop-filter: blur(var(--blur-glass));
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-nav);
  z-index: 50;
  padding: 1rem;
  display: none;
  animation: kh-fade-in-up 0.2s ease-out;
}
.kh-search-container:focus-within .kh-search-dropdown {
  display: block;
}
.kh-search-suggestion {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  padding: 0.625rem 0.875rem;
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  text-decoration: none;
  font-size: var(--text-sm);
  transition: background 0.15s ease;
}
.kh-search-suggestion:hover {
  background: var(--color-brand-muted);
  color: var(--color-brand-foreground);
}

/* ─── County Hubs ─── */
.kh-county-strip {
  display: flex;
  gap: 0.5rem;
  justify-content: center;
  flex-wrap: wrap;
  margin-top: 1rem;
}
.kh-county-chip {
  padding: 0.25rem 0.75rem;
  border-radius: var(--radius-full);
  background: rgba(255,255,255,0.1);
  color: #fff;
  font-size: var(--text-xs);
  backdrop-filter: blur(4px);
  cursor: pointer;
  transition: all 0.2s ease;
}
.kh-county-chip:hover {
  background: var(--color-brand);
  transform: translateY(-1px);
}

/* ─── Flash Deals & Urgency ─── */
.kh-flash-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 1.5rem;
  margin-bottom: 3rem;
}
.kh-flash-card {
  display: flex;
  gap: 1rem;
  padding: 1rem;
  background: var(--color-surface-elevated);
  border: 1px solid var(--color-brand);
  border-radius: var(--radius-xl);
  position: relative;
  overflow: hidden;
}
.kh-flash-badge {
  position: absolute;
  top: 0;
  right: 0;
  background: var(--color-brand);
  color: #fff;
  font-size: 0.65rem;
  font-weight: bold;
  padding: 0.25rem 0.75rem;
  border-bottom-left-radius: var(--radius-md);
  z-index: 10;
}
.kh-flash-img {
  width: 100px;
  height: 100px;
  border-radius: var(--radius-lg);
  object-fit: cover;
}
.kh-flash-info {
  flex: 1;
  display: flex;
  flex-direction: column;
  justify-content: center;
}
.kh-progress-bar {
  height: 0.4rem;
  background: var(--color-border-subtle);
  border-radius: var(--radius-full);
  margin-top: 0.5rem;
  overflow: hidden;
}
.kh-progress-fill {
  height: 100%;
  background: linear-gradient(to right, var(--color-brand), var(--color-accent));
  border-radius: var(--radius-full);
}
.kh-countdown {
  display: flex;
  gap: 0.25rem;
  margin-top: 0.5rem;
}
.kh-countdown-unit {
  background: var(--color-surface);
  padding: 0.15rem 0.35rem;
  border-radius: var(--radius-sm);
  font-size: 0.7rem;
  font-weight: bold;
  color: var(--color-brand);
}

/* ─── Featured Vendor ─── */
.kh-vendor-spotlight {
  margin: 6rem 0;
  display: grid;
  grid-template-columns: 1fr;
  gap: 3rem;
  align-items: center;
}
.kh-vendor-profile-card {
  background: var(--color-surface-glass);
  -webkit-backdrop-filter: blur(var(--blur-glass));
  backdrop-filter: blur(var(--blur-glass));
  border: 1px solid var(--color-border-subtle);
  border-radius: var(--radius-xl);
  padding: 2.5rem;
  display: flex;
  flex-direction: column;
  gap: 1.5rem;
  box-shadow: var(--shadow-nav);
}
.kh-vendor-avatar-row {
  display: flex;
  align-items: center;
  gap: 1.25rem;
}
.kh-vendor-avatar {
  width: 4.5rem;
  height: 4.5rem;
  border-radius: var(--radius-full);
  border: 2px solid var(--color-brand);
  object-fit: cover;
}
.kh-vendor-stats {
  display: flex;
  gap: 1.5rem;
}
.kh-vendor-stat-item {
  display: flex;
  flex-direction: column;
}
.kh-vendor-stat-val {
  font-size: var(--text-lg);
  font-weight: var(--font-weight-bold);
  color: var(--color-text-primary);
}
.kh-vendor-stat-label {
  font-size: var(--text-xs);
  color: var(--color-text-secondary);
}
@media (min-width: 980px) {
  .kh-vendor-spotlight {
    grid-template-columns: 1fr 1.5fr;
  }
}

/* ─── Testimonials ─── */
.kh-testimonial-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 2rem;
  margin: 4rem 0;
}
.kh-testimonial-card {
  padding: 2rem;
  background: var(--color-surface-elevated);
  border-radius: var(--radius-xl);
  border: 1px solid var(--color-border-subtle);
  display: flex;
  flex-direction: column;
  gap: 1rem;
}
.kh-testimonial-author {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-top: auto;
}
.kh-testimonial-avatar {
  width: 2.5rem;
  height: 2.5rem;
  border-radius: var(--radius-full);
  object-fit: cover;
}

/* ─── Newsletter ─── */
.kh-newsletter-section {
  margin: 6rem 0;
  padding: 4rem 2rem;
  background: linear-gradient(135deg, var(--color-brand-muted), var(--color-surface-elevated));
  border-radius: var(--radius-2xl);
  text-align: center;
  border: 1px solid var(--color-brand-muted);
}
.kh-newsletter-form {
  max-width: 30rem;
  margin: 2rem auto 0;
  display: flex;
  gap: 0.5rem;
}
@media (max-width: 480px) {
  .kh-newsletter-form {
    flex-direction: column;
  }
}
"#;
