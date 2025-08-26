(() => {
  if (window.parent?.oidcUserManager) {
    window.parent.oidcUserManager.signinSilentCallback();
  }
})();
