(() => {
  if (window.parent?.oidcUserManager) {
    console.log("Processing silent renew callback");
    window.parent.oidcUserManager
      .signinSilentCallback()
      .then(() => {
        console.log("Silent renew completed successfully");
      })
      .catch((error) => {
        console.error("Silent renew error:", error);
      });
  } else {
    console.error("Unable to access UserManager from parent window");
  }
})();
