initSidebarItems({"enum":[["CreateNamespacedPodDisruptionBudgetResponse","Use `<CreateNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::create_namespaced_pod_disruption_budget`]"],["CreateNamespacedPodEvictionResponse","Use `<CreateNamespacedPodEvictionResponse as Response>::try_from_parts` to parse the HTTP response body of [`Eviction::create_namespaced_pod_eviction`]"],["CreatePodSecurityPolicyResponse","Use `<CreatePodSecurityPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodSecurityPolicy::create_pod_security_policy`]"],["DeleteCollectionNamespacedPodDisruptionBudgetResponse","Use `<DeleteCollectionNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::delete_collection_namespaced_pod_disruption_budget`]"],["DeleteCollectionPodSecurityPolicyResponse","Use `<DeleteCollectionPodSecurityPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodSecurityPolicy::delete_collection_pod_security_policy`]"],["DeleteNamespacedPodDisruptionBudgetResponse","Use `<DeleteNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::delete_namespaced_pod_disruption_budget`]"],["DeletePodSecurityPolicyResponse","Use `<DeletePodSecurityPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodSecurityPolicy::delete_pod_security_policy`]"],["ListNamespacedPodDisruptionBudgetResponse","Use `<ListNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::list_namespaced_pod_disruption_budget`]"],["ListPodDisruptionBudgetForAllNamespacesResponse","Use `<ListPodDisruptionBudgetForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::list_pod_disruption_budget_for_all_namespaces`]"],["ListPodSecurityPolicyResponse","Use `<ListPodSecurityPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodSecurityPolicy::list_pod_security_policy`]"],["PatchNamespacedPodDisruptionBudgetResponse","Use `<PatchNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::patch_namespaced_pod_disruption_budget`]"],["PatchNamespacedPodDisruptionBudgetStatusResponse","Use `<PatchNamespacedPodDisruptionBudgetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::patch_namespaced_pod_disruption_budget_status`]"],["PatchPodSecurityPolicyResponse","Use `<PatchPodSecurityPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodSecurityPolicy::patch_pod_security_policy`]"],["ReadNamespacedPodDisruptionBudgetResponse","Use `<ReadNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::read_namespaced_pod_disruption_budget`]"],["ReadNamespacedPodDisruptionBudgetStatusResponse","Use `<ReadNamespacedPodDisruptionBudgetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::read_namespaced_pod_disruption_budget_status`]"],["ReadPodSecurityPolicyResponse","Use `<ReadPodSecurityPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodSecurityPolicy::read_pod_security_policy`]"],["ReplaceNamespacedPodDisruptionBudgetResponse","Use `<ReplaceNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::replace_namespaced_pod_disruption_budget`]"],["ReplaceNamespacedPodDisruptionBudgetStatusResponse","Use `<ReplaceNamespacedPodDisruptionBudgetStatusResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::replace_namespaced_pod_disruption_budget_status`]"],["ReplacePodSecurityPolicyResponse","Use `<ReplacePodSecurityPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodSecurityPolicy::replace_pod_security_policy`]"],["WatchNamespacedPodDisruptionBudgetResponse","Use `<WatchNamespacedPodDisruptionBudgetResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::watch_namespaced_pod_disruption_budget`]"],["WatchPodDisruptionBudgetForAllNamespacesResponse","Use `<WatchPodDisruptionBudgetForAllNamespacesResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodDisruptionBudget::watch_pod_disruption_budget_for_all_namespaces`]"],["WatchPodSecurityPolicyResponse","Use `<WatchPodSecurityPolicyResponse as Response>::try_from_parts` to parse the HTTP response body of [`PodSecurityPolicy::watch_pod_security_policy`]"]],"struct":[["AllowedCSIDriver","AllowedCSIDriver represents a single inline CSI Driver that is allowed to be used."],["AllowedFlexVolume","AllowedFlexVolume represents a single Flexvolume that is allowed to be used."],["AllowedHostPath","AllowedHostPath defines the host volume conditions that will be enabled by a policy for pods to use. It requires the path prefix to be defined."],["CreateNamespacedPodDisruptionBudgetOptional","Optional parameters of [`PodDisruptionBudget::create_namespaced_pod_disruption_budget`]"],["CreateNamespacedPodEvictionOptional","Optional parameters of [`Eviction::create_namespaced_pod_eviction`]"],["CreatePodSecurityPolicyOptional","Optional parameters of [`PodSecurityPolicy::create_pod_security_policy`]"],["Eviction","Eviction evicts a pod from its node subject to certain policies and safety constraints. This is a subresource of Pod.  A request to cause such an eviction is created by POSTing to .../pods/<pod name>/evictions."],["FSGroupStrategyOptions","FSGroupStrategyOptions defines the strategy type and options used to create the strategy."],["HostPortRange","HostPortRange defines a range of host ports that will be enabled by a policy for pods to use.  It requires both the start and end to be defined."],["IDRange","IDRange provides a min/max of an allowed range of IDs."],["PodDisruptionBudget","PodDisruptionBudget is an object to define the max disruption that can be caused to a collection of pods"],["PodDisruptionBudgetList","PodDisruptionBudgetList is a collection of PodDisruptionBudgets."],["PodDisruptionBudgetSpec","PodDisruptionBudgetSpec is a description of a PodDisruptionBudget."],["PodDisruptionBudgetStatus","PodDisruptionBudgetStatus represents information about the status of a PodDisruptionBudget. Status may trail the actual state of a system."],["PodSecurityPolicy","PodSecurityPolicy governs the ability to make requests that affect the Security Context that will be applied to a pod and container."],["PodSecurityPolicyList","PodSecurityPolicyList is a list of PodSecurityPolicy objects."],["PodSecurityPolicySpec","PodSecurityPolicySpec defines the policy enforced."],["ReadNamespacedPodDisruptionBudgetOptional","Optional parameters of [`PodDisruptionBudget::read_namespaced_pod_disruption_budget`]"],["ReadNamespacedPodDisruptionBudgetStatusOptional","Optional parameters of [`PodDisruptionBudget::read_namespaced_pod_disruption_budget_status`]"],["ReadPodSecurityPolicyOptional","Optional parameters of [`PodSecurityPolicy::read_pod_security_policy`]"],["ReplaceNamespacedPodDisruptionBudgetOptional","Optional parameters of [`PodDisruptionBudget::replace_namespaced_pod_disruption_budget`]"],["ReplaceNamespacedPodDisruptionBudgetStatusOptional","Optional parameters of [`PodDisruptionBudget::replace_namespaced_pod_disruption_budget_status`]"],["ReplacePodSecurityPolicyOptional","Optional parameters of [`PodSecurityPolicy::replace_pod_security_policy`]"],["RunAsGroupStrategyOptions","RunAsGroupStrategyOptions defines the strategy type and any options used to create the strategy."],["RunAsUserStrategyOptions","RunAsUserStrategyOptions defines the strategy type and any options used to create the strategy."],["RuntimeClassStrategyOptions","RuntimeClassStrategyOptions define the strategy that will dictate the allowable RuntimeClasses for a pod."],["SELinuxStrategyOptions","SELinuxStrategyOptions defines the strategy type and any options used to create the strategy."],["SupplementalGroupsStrategyOptions","SupplementalGroupsStrategyOptions defines the strategy type and options used to create the strategy."]]});