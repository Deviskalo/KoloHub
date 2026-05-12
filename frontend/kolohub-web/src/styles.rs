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
  .kh-product-grid { grid-template-columns: repeat(4, minmax(0, 1fr)); }
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
"#;
